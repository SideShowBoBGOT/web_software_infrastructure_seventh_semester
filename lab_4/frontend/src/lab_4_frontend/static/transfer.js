import axios from 'https://cdn.jsdelivr.net/npm/axios@1.3.5/+esm';

document.getElementById("groups").addEventListener("change", () => {
    const group = document.getElementById("groups").value
    axios.post("/transfer/changeGroup/", {
        "studentId": new URLSearchParams(document.location.search).get("studentId"),
        "newGroup": group.replace("group-", "")
    })
})