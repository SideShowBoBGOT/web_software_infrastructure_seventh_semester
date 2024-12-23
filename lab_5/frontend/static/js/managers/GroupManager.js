class GroupManager {
    constructor() {
        this.bindEvents();
        this.loadGroups();
    }

    bindEvents() {
        document.getElementById('addGroupForm')
            .addEventListener('submit', this.handleAddGroup.bind(this));
    }

    async loadGroups() {
        try {
            const groups = await API.groups.getAll();
            this.renderGroups(groups);
        } catch (error) {
            Utils.handleApiError(error, 'load groups');
        }
    }

    renderGroups(groups) {
        const table = document.getElementById('groupsTable');
        table.innerHTML = groups.map(group => `
            <tr>
                <td>${group.id}</td>
                <td>${group.name}</td>
                <td>
                    <a href="/update-group?id=${group.id}">Edit</a>
                    <button onclick="groupManager.deleteGroup(${group.id})">Delete</button>
                </td>
            </tr>
        `).join('');
    }

    async handleAddGroup(event) {
        event.preventDefault();
        const formData = new FormData(event.target);

        try {
            await API.groups.create({ name: formData.get('groupName') });
            Utils.showAlert('Group added successfully');
            event.target.reset();
            this.loadGroups();
        } catch (error) {
            Utils.handleApiError(error, 'add group');
        }
    }

    async deleteGroup(id) {
        if (await Utils.confirmDelete('group')) {
            try {
                await API.groups.delete(id);
                Utils.showAlert('Group deleted successfully');
                this.loadGroups();
            } catch (error) {
                Utils.handleApiError(error, 'delete group');
            }
        }
    }
}