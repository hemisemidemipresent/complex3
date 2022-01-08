let title = document.getElementById('title');
let shown = true;
function toggleBook() {
    shown = !shown;
    if (shown) title.style.display = 'block';
    else title.style.display = 'none';
}
