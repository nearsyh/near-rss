function initializeControl() {
    initializeLoginButton();
    initializeMenuButtons();
    initializeScrollListener();
    initializeShortcuts();
}

function initializeLoginButton() {
    document.getElementById('login')
        .addEventListener('click', _ => {
            let email = document.getElementById('email').value;
            let password = document.getElementById('password').value;
            login(email, password).catch(error => {
                    console.log("Fail to login", error);
                })
                .then(any => {
                    state.login = true;
                    showLoadingView();
                    return loadItems();
                })
                .then(any => {
                    refreshView();
                });
        })
}

function initializeMenuButtons() {
    document.getElementById('refresh-content')
        .addEventListener('click', async function(event) {
            await refreshPage();
        });

    document.getElementById('mark-all-read')
        .addEventListener('click', async function(event) {
            showLoadingView();
            await markItemsAsReadOnBothSides(state.items);
            setTimeout(() => {
                refreshPage();
            }, 100);
        });

    document.getElementById('add-subscription')
        .addEventListener('click', e => {
            // TODO: use the state to decide how to render the page.
            document.getElementById('add-subscription-dialog').hidden = false;
            state.addingSubscription = true;
            e.stopPropagation();
        });

    document.getElementById('login-container').addEventListener('click', e => {
        if (document.getElementById('add-subscription-dialog').hidden == false) {
            document.getElementById('add-subscription-dialog').hidden = true;
            state.addingSubscription = false
        }
    });

    document.getElementById('add').addEventListener('click', async function(event) {
        document.getElementById('add-subscription-dialog').hidden = true;
        state.addingSubscription = false;
        showSplash("Adding Subscription...");
        await addSubscription(
            document.getElementById('link').value,
            document.getElementById('title').value,
            document.getElementById('folder').value
        );
        await refreshPage();
    });
}

function initializeScrollListener() {
    document.addEventListener('scroll', _ => {
        scrolled();
    });
}

function initializeShortcuts() {
    document.addEventListener("keydown", async function(e) {
        if (!state.login || state.addingSubscription) {
            return;
        }
        if (e.key == 'v' && indexOfOpenItem() != -1) {
            window.open(openItemUrl());
            return;
        }
        if (e.key == 'r') {
            await refreshPage();
            return;
        }
        if (e.key == 'e') {
            itemOnExpand();
            refreshView();
            return;
        }
        if (e.key == 'z') {
            if (state.openItem) {
                await markItemsAsReadOnBothSides(itemsUntil(state.openItem));
                refreshView();
            }
            return;
        }

        var index = -1;
        if (e.key == 'j' || e.key == 'ArrayDown') {
            index = indexOfOpenItem() + 1;
        } else if (e.key == 'k' || e.key == 'ArrayUp') {
            index = indexOfOpenItem() - 1;
        }
        if (index >= 0 && index <= state.items.length - 1) {
            await itemOnSelect(state.items[index]);
            document.getElementById(state.items[index].id).scrollIntoView();
            window.scrollBy(0, -10);
        }
    });
}

async function refreshPage() {
    cleanUpState();
    showLoadingView();
    await loadItems();
    refreshView();
}

async function loadItems() {
    if (state.isLoadingNextPage) {
        return false;
    }
    state.isLoadingNextPage = true;
    await loadUnreadItems(state.nextOffset)
        .then(items => {
            addItems(items.items);
            state.nextOffset = items.nextPageOffset || "";
        });
    state.isLoadingNextPage = false;
    return true;
}

async function scrolled() {
    if (document.body.scrollHeight - window.scrollY < 1500) {
        if (state.nextOffset != "") {
            if (await loadItems()) {
                refreshView();
            }
        }
    }
}

async function itemOnExpand() {
    state.expandItem = (state.openItem !== undefined);
}

async function itemOnSelect(item) {
    state.expandItem = false;
    let oldOpenItem = state.openItem;
    if (isItemOpen(item)) {
        state.openItem = undefined;
    } else {
        state.openItem = item;
        // Don't wait to make it faster.
        markItemsAsReadOnBothSides([item]);
    }
    if (oldOpenItem) {
        refreshItemView(oldOpenItem);
    }
    refreshItemView(item);
}

async function markItemsAsReadOnBothSides(items) {
    items = items.filter(item => !isItemRead(item));
    markAsItemAsReadLocally(items);

    itemIds = items.map(item => item.id);
    await markItemsAsRead(itemIds).catch(err => {
        console.log(`Fail to mark item ${itemIds} as read remotely`);
    });
}