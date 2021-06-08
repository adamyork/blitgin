const express = require('express');
const app = express();
app.use(express.json());
app.use(express.urlencoded({
    extended: true
}));
app.use(express.static('public'));
const port = 8080;


app.listen(port, () => {
    console.log(`blitgin harness server running on http://localhost:${port}`)
});