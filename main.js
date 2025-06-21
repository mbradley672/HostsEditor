import { invoke } from '@tauri-apps/api/tauri';
import { ask, confirm, message } from '@tauri-apps/api/dialog';

// Global state
let hostsEntries = [];
let backups = [];
let isElevated = false;
let isTextEditorVisible = false;
let searchTerm = '';

// Initialize the application
async function init() {
    await checkElevation();
    await loadHostsFile();
    await loadBackups();
}

// Check if the application is running with elevated privileges
async function checkElevation() {
    try {
        isElevated = await invoke('check_elevation');
        updateElevationStatus();
    } catch (error) {
        console.error('Failed to check elevation:', error);
        updateElevationStatus();
    }
}

// Update the elevation status indicator
function updateElevationStatus() {
    const statusDot = document.getElementById('elevationStatus');
    const statusText = document.getElementById('elevationText');
    const elevateBtn = document.getElementById('elevateBtn');

    if (isElevated) {
        statusDot.classList.add('elevated');
        statusText.textContent = 'Running with admin privileges';
        elevateBtn.style.display = 'none';
    } else {
        statusDot.classList.remove('elevated');
        statusText.textContent = 'Running without admin privileges';
        elevateBtn.style.display = 'block';
    }
}

// Request elevation
window.requestElevation = async function() {
    try {
        const success = await invoke('request_elevation_command');
        if (success) {
            isElevated = true;
            updateElevationStatus();
            await message('Elevation granted successfully!', 'Success');
        }
    } catch (error) {
        console.error('Failed to request elevation:', error);
        await message(`Failed to request elevation: ${error}`, 'Error');
    }
}

// Load hosts file
window.loadHostsFile = async function() {
    try {
        setLoading(true);
        hostsEntries = await invoke('load_hosts_file');
        renderHostsEntries();
    } catch (error) {
        console.error('Failed to load hosts file:', error);
        await message(`Failed to load hosts file: ${error}`, 'Error');
    } finally {
        setLoading(false);
    }
}

// Save hosts file
window.saveHostsFile = async function() {
    try {
        if (!isElevated) {
            const shouldElevate = await ask(
                'Admin privileges are required to save the hosts file. Request elevation?',
                'Elevation Required'
            );
            if (!shouldElevate) {
                return;
            }
        }

        setLoading(true);
        await invoke('save_hosts_file', { entries: hostsEntries });
        await message('Hosts file saved successfully!', 'Success');
    } catch (error) {
        console.error('Failed to save hosts file:', error);
        await message(`Failed to save hosts file: ${error}`, 'Error');
    } finally {
        setLoading(false);
    }
}

// Add new hosts entry
window.addNewEntry = function() {
    const newEntry = {
        ip: '127.0.0.1',
        hostname: 'example.com',
        comment: '',
        enabled: true
    };
    hostsEntries.push(newEntry);
    renderHostsEntries();
}

// Remove hosts entry
window.removeEntry = function(index) {
    hostsEntries.splice(index, 1);
    renderHostsEntries();
}

// Toggle hosts entry
window.toggleEntry = function(index) {
    hostsEntries[index].enabled = !hostsEntries[index].enabled;
    renderHostsEntries();
}

// Update hosts entry field
window.updateEntry = function(index, field, value) {
    hostsEntries[index][field] = value;
}

// Render hosts entries
function renderHostsEntries() {
    const container = document.getElementById('hostsTable');
    container.innerHTML = '';

    hostsEntries.forEach((entry, index) => {
        // Apply search filter
        const matchesSearch = !searchTerm || 
            entry.ip.toLowerCase().includes(searchTerm) ||
            entry.hostname.toLowerCase().includes(searchTerm) ||
            entry.comment.toLowerCase().includes(searchTerm);

        const entryDiv = document.createElement('div');
        entryDiv.className = `hosts-entry ${entry.enabled ? '' : 'disabled'} ${matchesSearch ? '' : 'filtered-out'}`;
        
        entryDiv.innerHTML = `
            <div class="entry-header">
                <label class="toggle-switch">
                    <input type="checkbox" ${entry.enabled ? 'checked' : ''} 
                           onchange="toggleEntry(${index})">
                    <span class="slider"></span>
                </label>
                <div class="entry-actions">
                    <button class="btn btn-danger btn-small" onclick="removeEntry(${index})">
                        Remove
                    </button>
                </div>
            </div>
            <div class="entry-fields">
                <input type="text" class="entry-field" placeholder="IP Address" 
                       value="${entry.ip}" 
                       onchange="updateEntry(${index}, 'ip', this.value)">
                <input type="text" class="entry-field" placeholder="Hostname" 
                       value="${entry.hostname}" 
                       onchange="updateEntry(${index}, 'hostname', this.value)">
                <input type="text" class="entry-field" placeholder="Comment (optional)" 
                       value="${entry.comment}" 
                       onchange="updateEntry(${index}, 'comment', this.value)">
            </div>
        `;
        
        container.appendChild(entryDiv);
    });

    // Show count of filtered results
    const totalEntries = hostsEntries.length;
    const visibleEntries = hostsEntries.filter((entry, index) => {
        return !searchTerm || 
            entry.ip.toLowerCase().includes(searchTerm) ||
            entry.hostname.toLowerCase().includes(searchTerm) ||
            entry.comment.toLowerCase().includes(searchTerm);
    }).length;

    if (searchTerm && visibleEntries !== totalEntries) {
        const countDiv = document.createElement('div');
        countDiv.style.cssText = 'text-align: center; padding: 10px; color: #666; font-style: italic;';
        countDiv.textContent = `Showing ${visibleEntries} of ${totalEntries} entries`;
        container.appendChild(countDiv);
    }
}

// Create backup
window.createBackup = async function() {
    const backupName = document.getElementById('backupName').value.trim();
    
    if (!backupName) {
        await message('Please enter a backup name', 'Error');
        return;
    }

    try {
        setLoading(true);
        const backup = await invoke('create_backup', { name: backupName });
        document.getElementById('backupName').value = '';
        await loadBackups();
        await message(`Backup "${backup.name}" created successfully!`, 'Success');
    } catch (error) {
        console.error('Failed to create backup:', error);
        await message(`Failed to create backup: ${error}`, 'Error');
    } finally {
        setLoading(false);
    }
}

// Load backups
window.loadBackups = async function() {
    try {
        backups = await invoke('list_backups');
        renderBackups();
    } catch (error) {
        console.error('Failed to load backups:', error);
        await message(`Failed to load backups: ${error}`, 'Error');
    }
}

// Restore backup
window.restoreBackup = async function(backupName) {
    const shouldRestore = await confirm(
        `Are you sure you want to restore the backup "${backupName}"? This will overwrite your current hosts file.`,
        'Confirm Restore'
    );

    if (!shouldRestore) {
        return;
    }

    try {
        if (!isElevated) {
            const shouldElevate = await ask(
                'Admin privileges are required to restore a backup. Request elevation?',
                'Elevation Required'
            );
            if (!shouldElevate) {
                return;
            }
        }

        setLoading(true);
        hostsEntries = await invoke('restore_backup', { backupName });
        renderHostsEntries();
        await message(`Backup "${backupName}" restored successfully!`, 'Success');
    } catch (error) {
        console.error('Failed to restore backup:', error);
        await message(`Failed to restore backup: ${error}`, 'Error');
    } finally {
        setLoading(false);
    }
}

// Delete backup
window.deleteBackup = async function(backupName) {
    const shouldDelete = await confirm(
        `Are you sure you want to delete the backup "${backupName}"? This action cannot be undone.`,
        'Confirm Delete'
    );

    if (!shouldDelete) {
        return;
    }

    try {
        setLoading(true);
        await invoke('delete_backup', { backupName });
        await loadBackups();
        await message(`Backup "${backupName}" deleted successfully!`, 'Success');
    } catch (error) {
        console.error('Failed to delete backup:', error);
        await message(`Failed to delete backup: ${error}`, 'Error');
    } finally {
        setLoading(false);
    }
}

// Render backups
function renderBackups() {
    const container = document.getElementById('backupList');
    container.innerHTML = '';

    if (backups.length === 0) {
        container.innerHTML = '<p style="text-align: center; color: #666; padding: 20px;">No backups found</p>';
        return;
    }

    backups.forEach(backup => {
        const backupDiv = document.createElement('div');
        backupDiv.className = 'backup-item';
        
        backupDiv.innerHTML = `
            <div class="backup-name">${backup.name}</div>
            <div class="backup-date">${backup.created_at}</div>
            <div class="backup-actions">
                <button class="btn btn-small" onclick="restoreBackup('${backup.name}')">
                    Restore
                </button>
                <button class="btn btn-danger btn-small" onclick="deleteBackup('${backup.name}')">
                    Delete
                </button>
            </div>
        `;
        
        container.appendChild(backupDiv);
    });
}

// Set loading state
function setLoading(loading) {
    const container = document.querySelector('.container');
    if (loading) {
        container.classList.add('loading');
    } else {
        container.classList.remove('loading');
    }
}

// Search and filter functions
window.filterEntries = function() {
    const searchInput = document.getElementById('searchInput');
    searchTerm = searchInput.value.toLowerCase();
    renderHostsEntries();
}

window.clearSearch = function() {
    document.getElementById('searchInput').value = '';
    searchTerm = '';
    renderHostsEntries();
}

// Text editor functions
window.toggleTextEditor = function() {
    const textEditor = document.getElementById('textEditor');
    const hostsTable = document.getElementById('hostsTable');
    
    isTextEditorVisible = !isTextEditorVisible;
    
    if (isTextEditorVisible) {
        textEditor.classList.remove('hidden');
        hostsTable.style.display = 'none';
        loadRawText();
    } else {
        textEditor.classList.add('hidden');
        hostsTable.style.display = 'block';
    }
}

window.loadRawText = async function() {
    try {
        setLoading(true);
        const rawContent = await invoke('get_raw_hosts_content');
        document.getElementById('rawHostsText').value = rawContent;
    } catch (error) {
        console.error('Failed to load raw hosts content:', error);
        await message(`Failed to load raw hosts content: ${error}`, 'Error');
    } finally {
        setLoading(false);
    }
}

window.parseRawText = async function() {
    try {
        const rawContent = document.getElementById('rawHostsText').value;
        
        if (!isElevated) {
            const shouldElevate = await ask(
                'Admin privileges are required to save the hosts file. Request elevation?',
                'Elevation Required'
            );
            if (!shouldElevate) {
                return;
            }
        }

        setLoading(true);
        hostsEntries = await invoke('save_raw_hosts_content', { content: rawContent });
        
        // Switch back to normal view and refresh
        toggleTextEditor();
        renderHostsEntries();
        
        await message('Hosts file updated successfully from raw text!', 'Success');
    } catch (error) {
        console.error('Failed to save raw hosts content:', error);
        await message(`Failed to save raw hosts content: ${error}`, 'Error');
    } finally {
        setLoading(false);
    }
}

// Initialize the application when the page loads
document.addEventListener('DOMContentLoaded', init);
