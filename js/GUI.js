let title = document.getElementById('title');
let sidebar = document.getElementById('sidebar');

let titleShown = true;
let sidebarShown = false;
function toggleBook() {
    titleShown = !titleShown;
    if (titleShown) title.style.display = 'block';
    else title.style.display = 'none';
}

function helpClicked() {
    sidebarShown = !sidebarShown;
    if (sidebarShown) sidebar.style.display = 'block';
    else sidebar.style.display = 'none';
}
