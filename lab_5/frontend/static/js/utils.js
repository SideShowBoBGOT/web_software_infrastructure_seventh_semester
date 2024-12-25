const Utils = {
    showAlert(message, type = 'info') {
        alert(message); // Could be enhanced with a better UI component
    },

    handleApiError(error, context) {
        console.error(`Error in ${context}:`, error);
        this.showAlert(`Failed to ${context}: ${error.message}`);
    },

    async confirmDelete(entityType) {
        return confirm(`Are you sure you want to delete this ${entityType}?`);
    },

    populateGroupSelect(selectElement, groups, selectedValue = '') {
        selectElement.innerHTML = '';
        
        const placeholder = document.createElement('option');
        placeholder.value = '';
        placeholder.textContent = 'Select Group';
        placeholder.disabled = true;
        placeholder.selected = !selectedValue;
        selectElement.appendChild(placeholder);
        
        groups.forEach(group => {
            const option = document.createElement('option');
            option.value = group.id;
            option.textContent = group.name;
            option.selected = group.id === selectedValue;
            selectElement.appendChild(option);
        });
    }
};