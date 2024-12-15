import axios from 'https://cdn.jsdelivr.net/npm/axios@1.3.5/+esm';

document.getElementById("groups").addEventListener("change", () => {
    const group = document.getElementById("groups").value
    const groupId = group.replace("group-", "")
    const studentId =  new URLSearchParams(document.location.search).get("studentId")
    axios.put("/transfer/changeGroup/", {
        "studentId": studentId,
        "groupId": groupId
    })
})