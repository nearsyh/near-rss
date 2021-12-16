function showLoadingView(text = undefined) {
    hideElementById('non-login-container');
    hideElementById('login-container');
    document.getElementById('add-subscription-dialog').hidden = true;
    if (text) {
        document.getElementById('loading-text').textContent = text;
    } else {
        document.getElementById('loading-text').textContent = "Loading...";
    }
    showElementById('splash');
}

function hideLoadingView() {
    hideElementById('splash');
}

function refreshView() {
    hideLoadingView();
    if (state.login) {
        renderItems();
        showElementById('login-container');
    } else {
        showElementById('non-login-container');
    }
}

function hideElementById(id) {
    let element = document.getElementById(id);
    element.hidden = true;
    element.style.display = 'none';
}

function showElementById(id) {
    let element = document.getElementById(id);
    element.hidden = false;
    element.style.display = 'flex';
}

function refreshItemView(item) {
    let itemContainer = document.getElementById(item.id);
    document.getElementById('contents-list')
        .replaceChild(constructItemView(item), itemContainer);
}

function renderItems() {
    let itemsListContainer = document.getElementById('contents-list');
    itemsListContainer.innerHTML = '';
    if (state.items.length != 0) {
        for (const item of state.items) {
            itemsListContainer.appendChild(constructItemView(item));
        }
    } else {
        let noItems = document.createElement('div');
        noItems.textContent = "Nothing left";
        itemsListContainer.appendChild(noItems);
    }
}

function constructItemView(item) {
    var itemContainer = document.createElement('div');
    var itemContainerClass = 'item-container'
    if (isItemOpen(item)) {
        itemContainerClass = `${itemContainerClass} open`;
    } else if (isItemRead(item)) {
        itemContainerClass = `${itemContainerClass} read`;
    }
    itemContainer.setAttribute('class', itemContainerClass);
    itemContainer.setAttribute('id', item.id);

    itemContainer.appendChild(constructItemHeadView(item));
    if (isItemExpand(item)) {
        itemContainer.appendChild(constructItemBodyView(item));
    }

    return itemContainer;
}

function constructItemHeadView(item) {
    var itemHeadContainer = document.createElement('div');
    itemHeadContainer.setAttribute('class', 'item-head-container');

    var subscriptionTitleContainer = document.createElement('div');
    subscriptionTitleContainer.setAttribute('class', 'item-subscription-title');
    subscriptionTitleContainer.textContent = item.origin.title;
    itemHeadContainer.appendChild(subscriptionTitleContainer);

    var titleContainer = document.createElement('div');
    titleContainer.setAttribute('class', 'item-title');
    titleContainer.textContent = item.title;
    itemHeadContainer.appendChild(titleContainer);

    itemHeadContainer.addEventListener('click', async function(_) {
        await itemOnSelect(item);
    });

    // var dateContainer = document.createElement('div');
    // dateContainer.setAttribute('class', 'item-date');
    // dateContainer.textContent = getItemPublishedDateAsStr(item);
    // itemHeadContainer.appendChild(dateContainer);

    return itemHeadContainer;
}

function constructItemBodyView(item) {
    var itemBodyContainer = document.createElement('div');
    itemBodyContainer.setAttribute('class', 'item-body-container');
    itemBodyContainer.innerHTML = item.summary.content;
    return itemBodyContainer;
}


function getItemPublishedDateAsStr(item) {
    let date = new Date(item.published * 1000);
    let ye = new Intl.DateTimeFormat('en', { year: 'numeric' }).format(date);
    let mo = new Intl.DateTimeFormat('en', { month: 'numeric' }).format(date);
    let da = new Intl.DateTimeFormat('en', { day: '2-digit' }).format(date);
    let hour = new Intl.DateTimeFormat('en', { hour: '2-digit', hour12: false }).format(date);
    let minute = new Intl.DateTimeFormat('en', { minute: '2-digit' }).format(date);
    return `${ye}-${mo}-${da} ${hour}:${minute}`;
}