const API = {
    base_url: window.API_BASE_URL,

    async fetchJson(endpoint, options = {}) {
        try {
            const response = await fetch(`${this.base_url}${endpoint}`, options);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const text = await response.text();
            return text ? JSON.parse(text) : {};
        } catch (error) {
            console.error(`API Error (${endpoint}):`, error);
            throw error;
        }
    },

    groups: {
        getAll: async () => {
            return API.fetchJson('/api/groups');
        },
        get: async (id) => {
            return API.fetchJson(`/api/groups/${id}`);
        },
        create: async (data) => {
            return API.fetchJson('/api/groups', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(data)
            });
        },
        update: async (id, data) => {
            return API.fetchJson(`/api/groups/${id}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(data)
            });
        },
        delete: async (id) => {
            return API.fetchJson(`/api/groups/${id}`, {
                method: 'DELETE'
            });
        }
    },

    students: {
        getAll: async () => {
            return API.fetchJson('/api/students');
        },
        get: async (id) => {
            return API.fetchJson(`/api/students/${id}`);
        },
        create: async (formData) => {
            const response = await fetch(`${API.base_url}/api/students`, {
                method: 'POST',
                body: formData
            });
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            return response;
        },
        update: async (id, formData) => {
            const response = await fetch(`${API.base_url}/api/students/${id}`, {
                method: 'PUT',
                body: formData
            });
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status} ${response.body}`);
            }
            return response;
        },
        delete: async (id) => {
            return API.fetchJson(`/api/students/${id}`, {
                method: 'DELETE'
            });
        },
        async getImage(id) {
            try {
                const response = await fetch(`${API.base_url}/api/students/image/${id}`);
                
                if (!response.ok) {
                    console.warn(`No image found for student ${id}, status: ${response.status}`);
                    return null;
                }

                // Verify content type
                const contentType = response.headers.get('content-type');
                if (!contentType || !contentType.includes('image/')) {
                    console.error(`Invalid content type for student ${id}: ${contentType}`);
                    return null;
                }

                const blob = await response.blob();
                if (blob.size === 0) {
                    console.warn(`Empty image blob received for student ${id}`);
                    return null;
                }

                // Create and return blob URL
                return URL.createObjectURL(blob);
            } catch (error) {
                console.error(`Error fetching image for student ${id}:`, error);
                return null;
            }
        }
    }
};