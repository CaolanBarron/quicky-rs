
// Search bar enter listener
var searchbar = document.getElementById("searchbar");

searchbar.addEventListener("keypress", function(event) {
    if(event.key == "Enter"){
        event.preventDefault();
        search_quicky();
    }
})

function search_quicky() {
    let input = document.getElementById('searchbar').value
    console.log(input)
    window.location.href = "/search?cmd=" + input;
}