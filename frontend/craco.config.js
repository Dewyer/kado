const path = require("path");
module.exports = {
    plugins: [{ plugin: require('@semantic-ui-react/craco-less') }],
    resolve: {
        alias: {
            src:  path.resolve(__dirname, 'src/')
        }
    }
}
