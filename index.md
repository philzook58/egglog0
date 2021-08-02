---
title: "Egglog"
---

## Try It Out!

<script type="module">
        export { run };
        import init, { run_wasm } from './pkg/egglog.js';

        async function run() {
            await init();
            var query = document.getElementById("query").value;
            let example = `
                f(x) = x.
                /*
                g(X)=f(x):-z.
                f(X) = g(Q) :- Q = X, f(x).
                */
                y = x.
                plus(X,Y) <- plus(Y,X). 
                plus(b,q).
                ?- f(x) = x, x = x, y = x, plus(b,q) = plus(q,b), f(f(x)).
                `
            const result = run_wasm(query);
            console.log(result);
            document.getElementById("result").value = result;

        }
        window.run = run;
        //run();
</script>

<script>
function pickerbox(select){
    var xhr = new XMLHttpRequest();
    xhr.open('GET', `/egglog/examples/${select.value}`, true);

    // If specified, responseType must be empty string or "text"
    xhr.responseType = 'text';

    xhr.onload = function () {
        if (xhr.readyState === xhr.DONE) {
            if (xhr.status === 200) {
                console.log(xhr.response);
                console.log(xhr.responseText);
                document.getElementById("query").value = xhr.responseText;
            }
        }
    };

    xhr.send(null);
}
window.onload = () => {pickerbox(document.getElementById("examplepicker"))}
</script>

<textarea id="query" rows="20" style="width:100%">
</textarea>
<button onclick="run()">Run</button>
<select name="example" onchange="pickerbox(this)" id="examplepicker">
  <option value="basics.pl">Basics</option>
  <option value="cat1.pl">Pullback of Monic is Monic</option>
</select>
<textarea id="result" rows="20" style="width:100%"> </textarea>

# What is this?

A prolog like syntax for interfacing with the egg egraph library.

Github repo: <https://github.com/philzook58/egglog>
Read more here: <https://www.philipzucker.com/egglog-checkpoint/>

