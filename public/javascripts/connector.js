let endpoint = window.location.origin
let unreadLimit = 50;

function setTokenInCookie(token) {
    document.cookie = `token=${token}`;
}

function getTokenInCookie() {
    var cookies = document.cookie.split(";");
    for (const cookie of cookies) {
        let parts = cookie.split("=");
        if (parts[0].trim() == "token") {
            return parts[1].trim();
        }
    }
    return "";
}

function isLogin() {
    return getTokenInCookie() != "";
}

function constructHeader() {
    return {
        'Authorization': `GoogleLogin auth=${getTokenInCookie()}`
    }
}

function handleResponseIfNotOk(response) {
    if (response.status == 403) {
        setTokenInCookie("");
        state.login = false;
        refreshView();
    }
    if (!response.ok) {
        throw `status ${response.status}, response: ${response.text}`;
    }
}

function login(email, password) {
    let data = {
        body: `Email=${email}&Passwd=${password}`,
        headers: {
            'content-type': 'application/x-www-form-urlencoded'
        },
        method: 'POST',
        mode: 'cors',
        redirect: 'follow',
    };
    return fetch(`${endpoint}/accounts/ClientLogin`, data)
        .then(response => {
            return response.text();
        })
        .then(text => {
            let token = text.split("\n")[2].split("=")[1];
            setTokenInCookie(token);
        });
}

function loadUnreadItems(offset) {
    let url = offset ? `${endpoint}/api/unread?offset=${offset}&limit=${unreadLimit}` : `${endpoint}/api/unread?limit=${unreadLimit}`;
    return fetch(url, { headers: constructHeader(), mode: 'cors' })
        .then(response => {
            handleResponseIfNotOk(response);
            return response.json();
        }).catch(error => {
            console.log("Fail to load unread items", error);
        });
}

function markItemsAsRead(ids) {
    return fetch(`${endpoint}/api/markAsRead`, {
            headers: {
                ...constructHeader(),
                'content-type': 'application/json'
            },
            method: 'POST',
            mode: 'cors',
            redirect: 'follow',
            body: JSON.stringify({
                ids: ids
            })
        })
        .then(response => {
            handleResponseIfNotOk(response);
            return;
        })
        .catch(error => {
            console.log("Fail to mark items as read", error);
        });
}

function addSubscription(link, title, folder) {
    return fetch(`${endpoint}/api/addSubscription`, {
            headers: {
                ...constructHeader(),
                'content-type': 'application/json'
            },
            method: 'POST',
            mode: 'cors',
            redirect: 'follow',
            body: JSON.stringify({
                link,
                title,
                folder
            })
        })
        .then(response => {
            handleResponseIfNotOk(response);
            return;
        })
        .catch(error => {
            console.log("Fail to add subscription", error);
        });
}