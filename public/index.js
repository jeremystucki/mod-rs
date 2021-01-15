const js = import('./node_modules/mod-rs');

js.then(js => {
    document.getElementById('input').addEventListener('input', () => {
        let input = document.getElementById('input').value;
        let mod = document.getElementById('mod').value;
        document.getElementById('output').textContent = js.calculate_output(input, mod);
    });
});
