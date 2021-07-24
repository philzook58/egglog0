
# Egglog

<script type="module">
        import init, { run_wasm } from './pkg/egglog.js';

        async function run() {
            await init();
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
            const result = run_wasm(example);
            console.log(result);

        }

        run();
</script>