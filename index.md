
# 

<script type="module">
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

        //run();
</script>

<textarea id="query" rows="20" style="width:100%"> 
f(x) = x.
y = x.
plus(X,Y) <- plus(Y,X). 
plus(b,q).
?- f(x) = x, x = x, y = x, plus(b,q) = plus(q,b), f(f(x)).
</textarea>
<button onclick="run()">Run</button>
<textarea id="result" rows="20" style="width:100%">  </result>