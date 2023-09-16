
let quicky_options = [
    {
        display: `<span class="highlighted-color">r</span>eddit`,
        command: "r ",
        arguments: [" generic search", "/ subreddit","@ user",]
    },
    {
        display: `<span class="highlighted-color">tw</span>itter`,
        command: "tw ",
        arguments: [" generic search", "@ user"]
    },
    {
        display: `<span class="highlighted-color">y</span>ou<span class="highlighted-color">t</span>ube`,
        command: "yt ",
        arguments: [" generic search", "@ user"]
    },
    {
        display: `<span class="highlighted-color">poro</span>fessor`,
        command: "poro ",
        arguments: [" user"]
    },
    {
        display: `<span class="highlighted-color">am</span>azon`,
        command: "am ",
        arguments: [" generic search"]
    },
    {
        display: `<span class="highlighted-color">g</span>it<span class="highlighted-color">h</span>ub`,
        command: "gh ",
        arguments: [" user"]
    },
    {
        display: `<span class="highlighted-color">m</span>oba<span class="highlighted-color">f</span>ire`,
        command: "mf ",
        arguments: [" champion"]
    },
    {
        display: `<span class="highlighted-color">pi</span>nterest`,
        command: "pi ",
        arguments: [" generic search"]
    },
    {
        display: `<span class="highlighted-color">pr</span>otonmail`,
        command: "pr",
        arguments: [" generic search"]
    },
    {
        display: `<span class="highlighted-color">st</span>tack overflow`,
        command: "s ",
        arguments: [" generic search"]
    },
    {
      display: `<span class="highlighted-color">h</span>ow<span class="highlighted-color">l</span>ong<span class="highlighted-color">t</span>o<span class="highlighted-color">b</span>eat`,
      command: "hltb ",
      arguments: [" generic search"]
    },
   
]

// Search bar enter listener
var searchbar = document.getElementById("searchbar");

searchbar.addEventListener("keydown", function(event) {
    
    set_preview(event.key);

    if(event.key == "Enter"){
        event.preventDefault();
        search_quicky();
    }
})

function set_preview(key) {
    if(key.length > 1) return;
    console.log(searchbar.value + key)
    var commandsContainer = document.getElementById('commands-container');
    commandsContainer.innerHTML = "";
    var argumentsContainer = document.getElementById('arguments-container');
    argumentsContainer.innerHTML = "";

    var successes = [];
    quicky_options.forEach(element => {
       if(element.command.includes(searchbar.value + key)) {
            commandsContainer.innerHTML += "<span class='commands-text'>" + element.display + "</span>";
            successes.push(element)
       } 
    });

    if (successes.length == 1) {
        successes[0].arguments.forEach( element => {
            argumentsContainer.innerHTML+= "<span class='arguments-text'>" + element + "</span>";
        }
        )
    }
}

function search_quicky() {
    let input = document.getElementById('searchbar').value
    console.log(input)
    window.location.href = "/search?cmd=" + input;
}