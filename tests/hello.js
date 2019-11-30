import {a_string} from './test.js';

let abc=a_string;

function delay(t, v) {
   return new Promise(function(resolve) { 
       setTimeout(resolve.bind(null, v), t)
   });
}

let r = get_url(17);
console.log("get_url: ", r);

let r2 = hello("friend");
console.log(r2);

let r3 = notifyOnThingStatesChange("thing_id", thing_id => {
    console.log("thing_id");
});

console.log("notifyOnThingStatesChange", r3);

delay(1000).then(function() {
globalThis.output="timeout";
        console.log(globalThis.output)
    });


globalThis.output="abc";
console.log("Hello World", abc);
