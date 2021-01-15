const js = import('./node_modules/mod-rs');

js.then(js => {
    document.getElementById('input').addEventListener('input', inputHandler);
    document.getElementById('mod').addEventListener('input', inputHandler);

    function inputHandler() {
        let input = document.getElementById('input').value;
        let mod = document.getElementById('mod').value;

        try {
            document.getElementById('output').textContent = js.calculate_output(input, mod);
        } catch (err) {
            document.getElementById('output').textContent = err;
        }
    }
});
