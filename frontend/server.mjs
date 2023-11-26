import express from 'express'
import { globSync } from 'glob'

const extractOutcomeId = (filename) => {
    const pattern = /\.\.\/generated\/\d+-(\d+)\.png/
    const matches = filename.match(pattern)
    return matches[1]
}

const main = async () => {
    const app = express()
    app.use('/generated', express.static('../generated', { etag: false }))
    app.get('/events/:eventId', (req, res) => {
        const { eventId } = req.params
        const filenames = globSync(`../generated/${eventId}-*.png`)
        const html = `
            <!doctype html>
            <html>
                <head>
                    <style>
                        body {
                            display: grid;
                            grid-template-columns: repeat(2, 1fr); /* Two equal-width columns */
                            gap: 16px; /* Spacing between items */
                            width: 100%;
                            margin: 0 auto; /* Center the container */
                            height: 100vh; /* Full viewport height */
                            align-items: center; /* Vertical alignment */
                        }
        
                        div {
                            padding: 16px;
                            display: flex; /* Using Flexbox */
                            flex-direction: column; /* Stack content vertically */
                            justify-content: center; /* Center content vertically */
                            align-items: center; /* Center content horizontally */
                            height: 100%; /* Full height */
                        }
        
                        img {
                            max-width: 100%; /* Images fit within their div */
                            height: auto;
                        }
                    </style>
                </head>
                <body>
                    ${filenames.map(filename => `<div>${extractOutcomeId(filename)}<br><img src="../../generated/${eventId}-${extractOutcomeId(filename)}.png"></div>`).join('\n')}
            
                    <script type="text/javascript">
                        setTimeout(() => {
                            window.location.reload()
                        }, 5000)
                    </script>
                </body>
            </html>
        `
        res
            .status(200)
            .set('Content-Type', 'text/html')
            .send(html)
    })
    await new Promise(resolve => app.listen(8080, resolve))
}

main()
