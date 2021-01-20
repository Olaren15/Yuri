module.exports = {
    devServer: {
        proxy: {
            '^/api': {
                target: 'http://localhost:6969',
                changeOrigin: true
            },
        }
    }
}