// State
let scanResults = null;
let selectedProjects = new Set();
let selectedCaches = new Set();
let charts = {};
let currentProjectId = null; // For project detail modal

// DOM Elements
const scanBtn = document.getElementById('scan-btn');
const scanPath = document.getElementById('scan-path');
const userCaches = document.getElementById('user-caches');
const cacheOnly = document.getElementById('cache-only');
const statusBar = document.getElementById('status-bar');
const statusText = document.getElementById('status-text');
const resultsSection = document.getElementById('results');

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    scanBtn.addEventListener('click', startScan);

    // Check for existing results
    checkStatus();
});

// Utility: Escape HTML to prevent XSS
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// API Functions
async function api(endpoint, options = {}) {
    const response = await fetch(endpoint, {
        ...options,
        headers: {
            'Content-Type': 'application/json',
            ...options.headers
        }
    });
    return response.json();
}

async function checkStatus() {
    try {
        const result = await api('/api/status');
        if (result.success && result.data.has_results) {
            await loadResults();
        }
    } catch (e) {
        console.error('Status check failed:', e);
    }
}

async function startScan() {
    const path = scanPath.value || '.';
    const includeUserCaches = userCaches.checked;
    const onlyCaches = cacheOnly.checked;

    // Update UI
    scanBtn.disabled = true;
    scanBtn.querySelector('.btn-text').classList.add('hidden');
    scanBtn.querySelector('.btn-loading').classList.remove('hidden');
    statusBar.classList.remove('hidden');
    resultsSection.classList.add('hidden');

    // Reset progress display
    updateProgressDisplay({
        message: 'Starting scan...',
        projects_found: 0,
        caches_found: 0,
        total_size_display: '0 B'
    });

    try {
        const params = new URLSearchParams({
            path,
            user_caches: includeUserCaches,
            cache_only: onlyCaches
        });

        // Start SSE connection for progress updates
        const eventSource = new EventSource(`/api/scan/progress`);

        eventSource.onmessage = (event) => {
            try {
                const progress = JSON.parse(event.data);
                updateProgressDisplay(progress);

                if (progress.complete) {
                    eventSource.close();
                    loadResults();
                }
            } catch (e) {
                console.error('Progress parse error:', e);
            }
        };

        eventSource.onerror = (e) => {
            console.error('SSE error:', e);
            eventSource.close();
            // Fallback to polling if SSE fails
            pollForResults();
        };

        // Trigger the scan
        await api(`/api/scan?${params}`);

    } catch (e) {
        console.error('Scan failed:', e);
        statusText.textContent = 'Scan failed: ' + e.message;
        resetScanButton();
    }
}

function updateProgressDisplay(progress) {
    const message = progress.message || 'Scanning...';
    const projects = progress.projects_found || 0;
    const caches = progress.caches_found || 0;
    const size = progress.total_size_display || '0 B';

    // Build status text
    let text = message;

    // Add counts if we have any findings
    if (projects > 0 || caches > 0) {
        const parts = [];
        if (projects > 0) parts.push(`${projects} project${projects !== 1 ? 's' : ''}`);
        if (caches > 0) parts.push(`${caches} cache${caches !== 1 ? 's' : ''}`);
        text = `${message} (${parts.join(', ')} found - ${size})`;
    }

    statusText.textContent = text;

    // Show current path if available (truncated)
    if (progress.current_path && !progress.complete) {
        const pathText = truncatePath(progress.current_path, 60);
        statusText.textContent = `${text}\n📁 ${pathText}`;
    }
}

function truncatePath(path, maxLen) {
    if (path.length <= maxLen) return path;
    const parts = path.split('/');
    let result = '';
    for (let i = parts.length - 1; i >= 0; i--) {
        const test = parts.slice(i).join('/');
        if (test.length > maxLen - 3) {
            return '...' + result;
        }
        result = '/' + test;
    }
    return path.substring(0, maxLen - 3) + '...';
}

async function pollForResults() {
    const maxAttempts = 120; // 2 minutes
    let attempts = 0;

    while (attempts < maxAttempts) {
        try {
            const status = await api('/api/status');

            if (!status.success) {
                throw new Error(status.error);
            }

            if (!status.data.scanning && status.data.has_results) {
                await loadResults();
                return;
            }

            if (!status.data.scanning && !status.data.has_results) {
                statusText.textContent = 'No results found';
                resetScanButton();
                return;
            }
        } catch (e) {
            console.error('Poll error:', e);
        }

        await new Promise(resolve => setTimeout(resolve, 1000));
        attempts++;
    }

    statusText.textContent = 'Scan timed out';
    resetScanButton();
}

async function loadResults() {
    try {
        const result = await api('/api/results');

        if (!result.success) {
            throw new Error(result.error);
        }

        scanResults = result.data;
        renderResults();

        statusBar.classList.add('hidden');
        resultsSection.classList.remove('hidden');
        resetScanButton();
    } catch (e) {
        console.error('Load results failed:', e);
        statusText.textContent = 'Failed to load results';
    }
}

function resetScanButton() {
    scanBtn.disabled = false;
    scanBtn.querySelector('.btn-text').classList.remove('hidden');
    scanBtn.querySelector('.btn-loading').classList.add('hidden');
}

// Render Functions
function renderResults() {
    if (!scanResults) return;

    // Reset selections
    selectedProjects = new Set();
    selectedCaches = new Set();

    // Render summary
    renderSummary();

    // Render charts
    renderCharts();

    // Render largest items
    renderLargestItems();

    // Render lists
    renderProjectsList();
    renderCachesList();

    // Update selection info
    updateSelectionInfo();

    // Show/hide sections based on content
    document.getElementById('projects-section').classList.toggle('hidden', scanResults.projects.length === 0);
    document.getElementById('caches-section').classList.toggle('hidden', scanResults.caches.length === 0);
}

function renderSummary() {
    document.getElementById('total-projects').textContent = scanResults.summary.total_projects;
    document.getElementById('total-caches').textContent = scanResults.summary.total_caches;
    document.getElementById('total-size').textContent = scanResults.summary.total_size_display;
}

function renderCharts() {
    // Destroy existing charts
    Object.values(charts).forEach(chart => chart.destroy());
    charts = {};

    // Ecosystem colors for visual distinction
    const ecosystemColors = [
        '#3b82f6', // blue
        '#f97316', // orange (Rust)
        '#10b981', // green
        '#8b5cf6', // purple
        '#ec4899', // pink
        '#06b6d4', // cyan
        '#f59e0b', // amber
        '#ef4444', // red
        '#84cc16'  // lime
    ];

    const ecosystemData = scanResults.summary.by_ecosystem;
    const hasProjects = ecosystemData.length > 0;
    const hasCaches = scanResults.summary.by_cache_category.length > 0;

    // Show/hide chart cards based on data
    document.getElementById('ecosystem-chart-card').classList.toggle('hidden', !hasProjects);
    document.getElementById('top-projects-chart-card').classList.toggle('hidden', !hasProjects);
    document.getElementById('category-chart-card').classList.toggle('hidden', !hasCaches);

    // Chart 1: By Ecosystem (donut) - clickable to filter
    if (hasProjects) {
        const ecoCtx = document.getElementById('ecosystem-chart').getContext('2d');
        charts.ecosystem = new Chart(ecoCtx, {
            type: 'doughnut',
            data: {
                labels: ecosystemData.map(e => e.label),
                datasets: [{
                    data: ecosystemData.map(e => e.size),
                    backgroundColor: ecosystemColors.slice(0, ecosystemData.length),
                    borderWidth: 0
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        position: 'right',
                        labels: {
                            color: '#94a3b8',
                            usePointStyle: true,
                            padding: 12
                        }
                    },
                    tooltip: {
                        callbacks: {
                            label: (ctx) => {
                                const eco = ecosystemData[ctx.dataIndex];
                                return `${eco.label}: ${formatBytes(ctx.raw)} (${eco.count} project${eco.count !== 1 ? 's' : ''})`;
                            }
                        }
                    }
                },
                onClick: (event, elements) => {
                    if (elements.length > 0) {
                        const index = elements[0].index;
                        const ecosystem = ecosystemData[index].ecosystem;
                        scrollToProjectsByEcosystem(ecosystem);
                    }
                }
            }
        });
    }

    // Chart 2: Top Projects by Size (bar chart of individual projects)
    if (hasProjects) {
        const topProjects = [...scanResults.projects]
            .sort((a, b) => b.total_size - a.total_size)
            .slice(0, 10);

        const topCtx = document.getElementById('top-projects-chart').getContext('2d');
        charts.topProjects = new Chart(topCtx, {
            type: 'bar',
            data: {
                labels: topProjects.map(p => p.name.length > 20 ? p.name.substring(0, 20) + '...' : p.name),
                datasets: [{
                    data: topProjects.map(p => p.total_size),
                    backgroundColor: topProjects.map(p => {
                        // Color by ecosystem
                        const ecoIndex = ecosystemData.findIndex(e => e.ecosystem === p.project_type);
                        return ecosystemColors[ecoIndex >= 0 ? ecoIndex : 0];
                    }),
                    borderRadius: 4
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                indexAxis: 'y',
                plugins: {
                    legend: { display: false },
                    tooltip: {
                        callbacks: {
                            label: (ctx) => {
                                const project = topProjects[ctx.dataIndex];
                                return `${formatBytes(ctx.raw)} - ${project.type_label}`;
                            }
                        }
                    }
                },
                scales: {
                    x: {
                        ticks: {
                            callback: (val) => formatBytes(val),
                            color: '#94a3b8'
                        },
                        grid: { color: '#334155' }
                    },
                    y: {
                        ticks: { color: '#94a3b8' },
                        grid: { display: false }
                    }
                },
                onClick: (event, elements) => {
                    if (elements.length > 0) {
                        const index = elements[0].index;
                        const project = topProjects[index];
                        showProjectDetails(project.id);
                    }
                }
            }
        });
    }

    // Chart 3: By Cache Category
    if (hasCaches) {
        const catCtx = document.getElementById('category-chart').getContext('2d');
        charts.category = new Chart(catCtx, {
            type: 'bar',
            data: {
                labels: scanResults.summary.by_cache_category.map(c => c.label),
                datasets: [{
                    data: scanResults.summary.by_cache_category.map(c => c.size),
                    backgroundColor: '#8b5cf6',
                    borderRadius: 4
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                indexAxis: 'y',
                plugins: {
                    legend: { display: false },
                    tooltip: {
                        callbacks: {
                            label: (ctx) => formatBytes(ctx.raw)
                        }
                    }
                },
                scales: {
                    x: {
                        ticks: {
                            callback: (val) => formatBytes(val),
                            color: '#94a3b8'
                        },
                        grid: { color: '#334155' }
                    },
                    y: {
                        ticks: { color: '#94a3b8' },
                        grid: { display: false }
                    }
                }
            }
        });
    }
}

// Scroll to projects section and highlight projects of specific ecosystem
function scrollToProjectsByEcosystem(ecosystem) {
    const projectsSection = document.getElementById('projects-section');
    projectsSection.scrollIntoView({ behavior: 'smooth', block: 'start' });

    // Briefly highlight matching projects
    scanResults.projects.forEach(p => {
        const row = document.querySelector(`#project-${p.id}`)?.closest('.item-row');
        if (row) {
            if (p.project_type === ecosystem) {
                row.style.background = 'var(--accent-primary)';
                row.style.transition = 'background 0.3s ease';
                setTimeout(() => {
                    row.style.background = '';
                }, 1500);
            }
        }
    });
}

function renderLargestItems() {
    const container = document.getElementById('largest-items-list');
    const items = scanResults.summary.largest_items;

    // Clear container
    container.textContent = '';

    if (items.length === 0) {
        const p = document.createElement('p');
        p.className = 'text-muted';
        p.textContent = 'No items found';
        container.appendChild(p);
        return;
    }

    const medals = ['🥇', '🥈', '🥉'];

    items.forEach((item, i) => {
        const div = document.createElement('div');
        div.className = 'largest-item';

        const rank = document.createElement('span');
        rank.className = 'rank';
        rank.textContent = medals[i] || `${i + 1}.`;

        const icon = document.createElement('span');
        icon.className = 'icon';
        icon.textContent = item.icon;

        const name = document.createElement('span');
        name.className = 'name';
        name.textContent = item.name;

        const badge = document.createElement('span');
        badge.className = 'type-badge';
        badge.textContent = item.item_type;

        const size = document.createElement('span');
        size.className = 'size';
        size.textContent = item.size_display;

        div.appendChild(rank);
        div.appendChild(icon);
        div.appendChild(name);
        div.appendChild(badge);
        div.appendChild(size);
        container.appendChild(div);
    });
}

function renderProjectsList() {
    const container = document.getElementById('projects-list');
    container.textContent = '';

    if (scanResults.projects.length === 0) {
        const p = document.createElement('p');
        p.style.cssText = 'color: var(--text-muted); padding: 1rem;';
        p.textContent = 'No projects found';
        container.appendChild(p);
        return;
    }

    scanResults.projects.forEach(p => {
        const row = document.createElement('div');
        row.className = 'item-row clickable';
        row.dataset.projectId = p.id;

        const checkbox = document.createElement('input');
        checkbox.type = 'checkbox';
        checkbox.id = `project-${p.id}`;
        checkbox.checked = selectedProjects.has(p.id);
        checkbox.addEventListener('change', (e) => {
            e.stopPropagation();
            toggleProject(p.id);
        });
        checkbox.addEventListener('click', (e) => e.stopPropagation());

        const icon = document.createElement('span');
        icon.className = 'icon';
        icon.textContent = p.type_icon;

        const info = document.createElement('div');
        info.className = 'info';

        const nameDiv = document.createElement('div');
        nameDiv.className = 'name';
        nameDiv.textContent = p.name;
        const typeSpan = document.createElement('span');
        typeSpan.style.color = 'var(--text-muted)';
        typeSpan.textContent = ` (${p.type_label})`;
        nameDiv.appendChild(typeSpan);

        const pathDiv = document.createElement('div');
        pathDiv.className = 'path';
        pathDiv.textContent = p.path;

        // Show artifact count hint
        const artifactHint = document.createElement('div');
        artifactHint.className = 'path';
        artifactHint.style.color = 'var(--accent-primary)';
        artifactHint.textContent = `${p.artifacts.length} artifact${p.artifacts.length !== 1 ? 's' : ''} - click for details`;

        info.appendChild(nameDiv);
        info.appendChild(pathDiv);
        info.appendChild(artifactHint);

        const size = document.createElement('span');
        size.className = 'size';
        size.textContent = p.size_display;

        const expandIcon = document.createElement('span');
        expandIcon.className = 'expand-icon';
        expandIcon.textContent = '›';

        row.appendChild(checkbox);
        row.appendChild(icon);
        row.appendChild(info);
        row.appendChild(size);
        row.appendChild(expandIcon);

        // Click handler for showing project details
        row.addEventListener('click', () => showProjectDetails(p.id));

        container.appendChild(row);
    });
}

function renderCachesList() {
    const container = document.getElementById('caches-list');
    container.textContent = '';

    if (scanResults.caches.length === 0) {
        const p = document.createElement('p');
        p.style.cssText = 'color: var(--text-muted); padding: 1rem;';
        p.textContent = 'No caches found';
        container.appendChild(p);
        return;
    }

    scanResults.caches.forEach(c => {
        const row = document.createElement('div');
        row.className = 'item-row';

        const checkbox = document.createElement('input');
        checkbox.type = 'checkbox';
        checkbox.id = `cache-${c.id}`;
        checkbox.checked = selectedCaches.has(c.id);
        checkbox.addEventListener('change', () => toggleCache(c.id));

        const icon = document.createElement('span');
        icon.className = 'icon';
        icon.textContent = c.icon;

        const info = document.createElement('div');
        info.className = 'info';

        const nameDiv = document.createElement('div');
        nameDiv.className = 'name';
        nameDiv.textContent = c.label;

        const pathDiv = document.createElement('div');
        pathDiv.className = 'path';
        pathDiv.textContent = c.path;

        info.appendChild(nameDiv);
        info.appendChild(pathDiv);

        if (c.requires_app_closed) {
            const warning = document.createElement('div');
            warning.className = 'warning';
            warning.textContent = `⚠️ ${c.requires_app_closed}`;
            info.appendChild(warning);
        }

        const size = document.createElement('span');
        size.className = 'size';
        size.textContent = c.size_display;

        row.appendChild(checkbox);
        row.appendChild(icon);
        row.appendChild(info);
        row.appendChild(size);
        container.appendChild(row);
    });
}

// Selection Functions
function toggleProject(id) {
    if (selectedProjects.has(id)) {
        selectedProjects.delete(id);
    } else {
        selectedProjects.add(id);
    }
    updateSelectionInfo();
}

function toggleCache(id) {
    if (selectedCaches.has(id)) {
        selectedCaches.delete(id);
    } else {
        selectedCaches.add(id);
    }
    updateSelectionInfo();
}

function selectAll(type) {
    if (type === 'projects') {
        scanResults.projects.forEach(p => {
            selectedProjects.add(p.id);
            const el = document.getElementById(`project-${p.id}`);
            if (el) el.checked = true;
        });
    } else {
        scanResults.caches.forEach(c => {
            selectedCaches.add(c.id);
            const el = document.getElementById(`cache-${c.id}`);
            if (el) el.checked = true;
        });
    }
    updateSelectionInfo();
}

function deselectAll(type) {
    if (type === 'projects') {
        selectedProjects.clear();
        scanResults.projects.forEach(p => {
            const el = document.getElementById(`project-${p.id}`);
            if (el) el.checked = false;
        });
    } else {
        selectedCaches.clear();
        scanResults.caches.forEach(c => {
            const el = document.getElementById(`cache-${c.id}`);
            if (el) el.checked = false;
        });
    }
    updateSelectionInfo();
}

function updateSelectionInfo() {
    const count = selectedProjects.size + selectedCaches.size;
    let totalSize = 0;
    let totalFolders = 0;

    selectedProjects.forEach(id => {
        const project = scanResults.projects.find(p => p.id === id);
        if (project) {
            totalSize += project.total_size;
            totalFolders += project.artifacts.length;
        }
    });

    selectedCaches.forEach(id => {
        const cache = scanResults.caches.find(c => c.id === id);
        if (cache) {
            totalSize += cache.size;
            totalFolders += 1;
        }
    });

    document.getElementById('selection-count').textContent = `${count} item${count !== 1 ? 's' : ''} selected`;
    document.getElementById('selection-size').textContent = `(${formatBytes(totalSize)})`;

    // Enable/disable buttons
    document.getElementById('clean-btn').disabled = count === 0;
    document.getElementById('script-btn').disabled = count === 0;

    // Update deletion preview
    renderDeletionPreview(totalFolders);
}

function renderDeletionPreview(totalFolders) {
    const preview = document.getElementById('deletion-preview');
    const list = document.getElementById('deletion-preview-list');
    const summary = document.getElementById('deletion-summary');

    // Hide if nothing selected
    if (selectedProjects.size === 0 && selectedCaches.size === 0) {
        preview.classList.add('hidden');
        return;
    }

    preview.classList.remove('hidden');
    summary.textContent = `${totalFolders} folder${totalFolders !== 1 ? 's' : ''} to delete`;

    // Build the list
    list.innerHTML = '';

    // Add selected projects
    selectedProjects.forEach(id => {
        const project = scanResults.projects.find(p => p.id === id);
        if (!project) return;

        const group = document.createElement('div');
        group.className = 'deletion-group';

        const header = document.createElement('div');
        header.className = 'deletion-group-header';
        header.innerHTML = `
            <span class="project-name">${escapeHtml(project.type_icon)} ${escapeHtml(project.name)} <span style="color: var(--text-muted); font-weight: normal;">(${escapeHtml(project.type_label)})</span></span>
            <span class="project-size">${escapeHtml(project.size_display)}</span>
        `;

        const items = document.createElement('div');
        items.className = 'deletion-group-items';

        project.artifacts.forEach(artifact => {
            const item = document.createElement('div');
            item.className = 'deletion-item';
            item.innerHTML = `
                <span class="path">${escapeHtml(artifact.path)}</span>
                <span class="size">${escapeHtml(artifact.size_display)}</span>
            `;
            items.appendChild(item);
        });

        group.appendChild(header);
        group.appendChild(items);
        list.appendChild(group);
    });

    // Add selected caches
    selectedCaches.forEach(id => {
        const cache = scanResults.caches.find(c => c.id === id);
        if (!cache) return;

        const group = document.createElement('div');
        group.className = 'deletion-group';

        const header = document.createElement('div');
        header.className = 'deletion-group-header';
        header.innerHTML = `
            <span class="project-name">${escapeHtml(cache.icon)} ${escapeHtml(cache.label)} <span style="color: var(--text-muted); font-weight: normal;">(${escapeHtml(cache.category_label)})</span></span>
            <span class="project-size">${escapeHtml(cache.size_display)}</span>
        `;

        const items = document.createElement('div');
        items.className = 'deletion-group-items';

        const item = document.createElement('div');
        item.className = 'deletion-item';
        item.innerHTML = `
            <span class="path">${escapeHtml(cache.path)}</span>
            <span class="size">${escapeHtml(cache.size_display)}</span>
        `;
        items.appendChild(item);

        group.appendChild(header);
        group.appendChild(items);
        list.appendChild(group);
    });
}

// Action Functions
async function generateScript() {
    if (selectedProjects.size === 0 && selectedCaches.size === 0) {
        alert('Please select items first');
        return;
    }

    try {
        const result = await api('/api/generate-script', {
            method: 'POST',
            body: JSON.stringify({
                project_ids: Array.from(selectedProjects),
                cache_ids: Array.from(selectedCaches)
            })
        });

        if (!result.success) {
            throw new Error(result.error);
        }

        document.getElementById('script-content').textContent = result.data.script;
        document.getElementById('script-modal').classList.remove('hidden');
    } catch (e) {
        alert('Failed to generate script: ' + e.message);
    }
}

function cleanSelected() {
    if (selectedProjects.size === 0 && selectedCaches.size === 0) {
        alert('Please select items first');
        return;
    }

    const count = selectedProjects.size + selectedCaches.size;
    document.getElementById('confirm-message').textContent =
        `Are you sure you want to delete ${count} item${count !== 1 ? 's' : ''}?`;
    document.getElementById('confirm-modal').classList.remove('hidden');
}

async function confirmClean() {
    closeConfirmModal();

    try {
        const result = await api('/api/clean', {
            method: 'POST',
            body: JSON.stringify({
                project_ids: Array.from(selectedProjects),
                cache_ids: Array.from(selectedCaches)
            })
        });

        if (!result.success) {
            throw new Error(result.error);
        }

        const data = result.data;
        let message = `Deleted ${data.deleted_count} item(s), freed ${data.freed_display}`;

        if (data.errors.length > 0) {
            message += `\n\nErrors:\n${data.errors.join('\n')}`;
        }

        alert(message);

        // Refresh results
        await loadResults();
    } catch (e) {
        alert('Cleanup failed: ' + e.message);
    }
}

// Modal Functions
function closeModal() {
    document.getElementById('script-modal').classList.add('hidden');
}

function closeConfirmModal() {
    document.getElementById('confirm-modal').classList.add('hidden');
}

function copyScript() {
    const script = document.getElementById('script-content').textContent;
    navigator.clipboard.writeText(script).then(() => {
        alert('Script copied to clipboard!');
    }).catch(e => {
        console.error('Copy failed:', e);
    });
}

// Utility Functions
function formatBytes(bytes) {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

// Project Detail Modal Functions
function showProjectDetails(projectId) {
    const project = scanResults.projects.find(p => p.id === projectId);
    if (!project) return;

    currentProjectId = projectId;

    // Set modal title
    document.getElementById('project-modal-title').textContent =
        `${project.type_icon} ${project.name}`;

    // Build info section
    const infoContainer = document.getElementById('project-modal-info');
    infoContainer.innerHTML = `
        <span class="label">Type:</span>
        <span class="value">${escapeHtml(project.type_label)}</span>
        <span class="label">Path:</span>
        <span class="value">${escapeHtml(project.path)}</span>
        <span class="label">Total Size:</span>
        <span class="value size">${escapeHtml(project.size_display)}</span>
        <span class="label">Artifacts:</span>
        <span class="value">${project.artifacts.length}</span>
    `;

    // Build artifacts list
    const artifactsContainer = document.getElementById('project-modal-artifacts');
    artifactsContainer.innerHTML = '';

    project.artifacts.forEach(artifact => {
        const item = document.createElement('div');
        item.className = 'artifact-item';

        const info = document.createElement('div');
        info.className = 'artifact-info';

        const name = document.createElement('div');
        name.className = 'artifact-name';
        name.textContent = artifact.name;

        const path = document.createElement('div');
        path.className = 'artifact-path';
        path.textContent = artifact.path;

        info.appendChild(name);
        info.appendChild(path);

        const size = document.createElement('span');
        size.className = 'artifact-size';
        size.textContent = artifact.size_display;

        item.appendChild(info);
        item.appendChild(size);
        artifactsContainer.appendChild(item);
    });

    // Update select button
    const selectBtn = document.getElementById('project-modal-select');
    if (selectedProjects.has(projectId)) {
        selectBtn.textContent = 'Deselect from Cleanup';
        selectBtn.className = 'btn btn-secondary';
    } else {
        selectBtn.textContent = 'Select for Cleanup';
        selectBtn.className = 'btn btn-primary';
    }

    // Show modal
    document.getElementById('project-modal').classList.remove('hidden');
}

function closeProjectModal() {
    document.getElementById('project-modal').classList.add('hidden');
    currentProjectId = null;
}

function selectProjectFromModal() {
    if (currentProjectId === null) return;

    toggleProject(currentProjectId);

    // Update checkbox in the list
    const checkbox = document.getElementById(`project-${currentProjectId}`);
    if (checkbox) {
        checkbox.checked = selectedProjects.has(currentProjectId);
    }

    // Update button text
    const selectBtn = document.getElementById('project-modal-select');
    if (selectedProjects.has(currentProjectId)) {
        selectBtn.textContent = 'Deselect from Cleanup';
        selectBtn.className = 'btn btn-secondary';
    } else {
        selectBtn.textContent = 'Select for Cleanup';
        selectBtn.className = 'btn btn-primary';
    }
}

// Close modals on escape key
document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
        closeModal();
        closeConfirmModal();
        closeProjectModal();
    }
});

// Close modals on backdrop click
document.querySelectorAll('.modal').forEach(modal => {
    modal.addEventListener('click', (e) => {
        if (e.target === modal) {
            modal.classList.add('hidden');
        }
    });
});
