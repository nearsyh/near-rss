var state = {
    login: isLogin(),
    items: [],
    openItem: undefined,
    expandItem: false,
    nextOffset: undefined,
    isLoadingNextPage: false,
}

function cleanUpState() {
    state.items = [];
    state.openItem = undefined;
    state.expandItem = false;
    state.nextOffset = undefined;
    state.isLoadingNextPage = false;
}

function addItems(new_items) {
    state.items = state.items.concat(new_items);
}

function isItemRead(item) {
    return item.categories.includes('user/-/state/com.google/read');
}

function isItemOpen(item) {
    return state.openItem && state.openItem.id == item.id;
}

function isItemExpand(item) {
    return isItemOpen(item) && state.expandItem;
}

function itemsUntil(item) {
    let index = state.items.indexOf(item);
    return state.items.slice(0, index + 1);
}

function markAsItemAsReadLocally(items) {
    for (const item of items) {
        item.categories.push('user/-/state/com.google/read');
    }
}

function indexOfOpenItem() {
    return state.openItem ? state.items.indexOf(state.openItem) : -1;
}

function openItemUrl() {
    return state.openItem.canonical[0].href;
}