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
            var proof = document.getElementById("proofmode").checked;
            var graph = false;
            const result = run_wasm(query, proof, graph);
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
                //console.log(xhr.response);
                //console.log(xhr.responseText);
                document.getElementById("query").value = xhr.responseText;
            }
        }
    };

    xhr.send(null);
}
window.onload = () => {
    urlParams = new URLSearchParams(window.location.search);
    url_eaxmple = urlParams.get('example');

    picker = document.getElementById("examplepicker")
    if(url_eaxmple != null){
        picker.value = url_eaxmple;
    }
    pickerbox(picker)

    
    }
</script>

<textarea id="query" rows="20" style="width:100%">
</textarea>
<button onclick="run()">Run</button>
<select name="example" onchange="pickerbox(this)" id="examplepicker">
  <option value="talk.pl">PLDI Talk</option>
  <option value="basics.pl">Basics</option>
   <option value="datalog.pl">Datalog</option>
   <option value="ski.pl">SKI Combinators</option>
   <option value="lists.pl">Lists</option>
  <option value="arith.pl">Arithmetic</option>
  <option value="mem.pl">Memory/Arrays</option>
  <option value="cat1.pl">Pullback of Monic is Monic</option>
  <option value="id_unique.pl">Uniqueness of Identity</option>
  <option value="pb_compose.pl">Composition of Pullbacks</option>
</select>
<input type="checkbox" id="proofmode" name="proofmode" value="">
<label for="proofmode"> Proofs (Experimental) </label><br>
<textarea id="result" rows="20" style="width:100%"> </textarea>

# What is this?

A prolog like syntax for interfacing with the egg egraph library.

Github repo: <https://github.com/philzook58/egglog>
Read more here: 
- <https://www.philipzucker.com/egglog-checkpoint/> - Early version of egglog, motivations.
- <https://www.philipzucker.com/egglog2-monic/> - A simple category theory theorem about pullbacks.
- <https://www.philipzucker.com/egglog-3/> - Arithmetic, SKI combinator, datalog, lists examples
