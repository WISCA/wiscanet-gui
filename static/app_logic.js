var app_node_pairs = [];

function addAppNodePair(){
// Get selected Node
curr_node_id = $("#edgenodes :selected").val();
curr_node_name = $("#edgenodes :selected").text();
// Get selected Application
curr_app_id = $("#applications :selected").val();
curr_app_name = $("#applications :selected").text();

var curr_pair = {node_id:curr_node_id, node_name:curr_node_name, app_id:curr_app_id, app_name:curr_app_name, logic_id: app_node_pairs.length+1};

app_node_pairs.push(curr_pair);

$("#pairs").append(`
<div class='item'>
<div class='content'>
    <div class='header'>${curr_node_name}: ${curr_app_name}</div>
    <div class='description'>Logical ID = ${app_node_pairs.length}</div>
</div>
</div>`);

}

function sendConfig(){
    // Serialize app_node_pairs into JSON and POST to /genconfig
    var json_text = JSON.stringify(app_node_pairs);
    console.log(json_text);
    $.post("/genconfig",json_text,null, "json");
}