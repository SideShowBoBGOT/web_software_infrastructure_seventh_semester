class GroupUpdater {
    constructor() {
        this.groupId = new URLSearchParams(window.location.search).get('id');
        this.initialize();
    }

    async initialize() {
        if (!this.groupId) {
            Utils.showAlert('No group ID provided');
            window.location.href = '/groups';
            return;
        }

        this.bindEvents();
        await this.loadGroupData();
    }

    bindEvents() {
        document.getElementById('updateGroupForm')
            .addEventListener('submit', this.handleUpdateGroup.bind(this));
    }

    async loadGroupData() {
        try {
            const group = await API.groups.get(this.groupId);
            document.getElementById('groupId').value = group.id;
            document.getElementById('groupName').value = group.name;
        } catch (error) {
            Utils.handleApiError(error, 'load group data');
        }
    }

    async handleUpdateGroup(event) {
        event.preventDefault();
        const formData = new FormData(event.target);

        try {
            await API.groups.update(this.groupId, {
                name: formData.get('groupName')
            });
            Utils.showAlert('Group updated successfully');
            window.location.href = '/groups';
        } catch (error) {
            Utils.handleApiError(error, 'update group');
        }
    }
}