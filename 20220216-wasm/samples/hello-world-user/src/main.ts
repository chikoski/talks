import helloWorld, {greet, greet_with_alert} from "hello-world"

document.addEventListener("DOMContentLoaded", () => {
    main();
});

async function main(){
    await helloWorld();
    document.querySelector("#greet")
        ?.addEventListener("click", () => {
            console.log("greet");
            greet();        
    });

    document.querySelector("#greet-with-alert")
        ?.addEventListener("click", () => {
            greet_with_alert();
    });
}