//when the page is loaded, the function is called
$(document).ready(function() {
    document.getElementById("submit").addEventListener("click", function(event){
        submit();
    });
});

function submit() {
    let formdata;
    let form = document.getElementById("form");
    formdata = new FormData(form);
    
};