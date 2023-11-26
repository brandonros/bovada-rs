import express from 'express'
import fs from 'fs'
import { execa } from 'execa'

const parseEvents = (eventId) => {
    const output = fs.readFileSync(`../output-${eventId}.tsv`, 'utf8')
    const lines = output.split('\n')
    const headers = lines[0].split('\t')
    const rows = lines.slice(1)
    return rows.map(row => {
        const splitRow = row.split('\t')
        const mappedRow = {}
        for (let i = 0; i < headers.length; ++i) {
            const header = headers[i]
            mappedRow[header] = splitRow[i]
        }
        return mappedRow
    })
}

const extractOutcomes = (eventId) => {
    const events = parseEvents(eventId)
    const outcomesMap = {}
    const marketsEvents = events
        .filter(event => event.event_type === 'MarketsEvent')
        .map(event => JSON.parse(event.event))
    const gameLineMarketEvents = marketsEvents.filter(marketsEvent => marketsEvent.description === 'Game Lines')
    for (const gameLineMarketEvent of gameLineMarketEvents) {
        const moneylineMarket = gameLineMarketEvent.markets.find(market => market.description === 'Moneyline')
        if (moneylineMarket && moneylineMarket.period.description === 'Live Game') {
            for (const outcome of moneylineMarket.outcomes) {
                outcomesMap[outcome.id] = {
                    market: moneylineMarket,
                    outcome
                }
            }
        }
        const pointSpreadMarket = gameLineMarketEvent.markets.find(market => market.description === 'Point Spread')
        if (pointSpreadMarket && pointSpreadMarket.period.description === 'Live Game') {
            for (const outcome of pointSpreadMarket.outcomes) {
                outcomesMap[outcome.id] = {
                    market: pointSpreadMarket,
                    outcome
                }
            }
        }
        const totalSpreadMarket = gameLineMarketEvent.markets.find(market => market.description === 'Total')
        if (totalSpreadMarket && totalSpreadMarket.period.description === 'Live Game') {
            for (const outcome of totalSpreadMarket.outcomes) {
                outcomesMap[outcome.id] = {
                    market: totalSpreadMarket,
                    outcome
                }
            }
        }
    }
    return outcomesMap
}

const main = async () => {
    const app = express()
    app.use('/generated', express.static('../generated', { etag: false }))
    app.get('/events/:eventId', async (req, res) => {
        const { eventId } = req.params
        const outcomesMap = extractOutcomes(eventId)
        const outcomeIds = Object.keys(outcomesMap)
        for (const outcomeId of outcomeIds) {
            await execa('./scripts/extract.py', [eventId, outcomeId], { cwd: '../' })
            await execa('./scripts/plot.sh', [eventId, outcomeId], { cwd: '../' })
        }
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
                    ${outcomeIds.map(outcomeId => `<div>${`${outcomesMap[outcomeId].market.description} - ${outcomesMap[outcomeId].outcome.description}`}<br><img src="../../generated/${eventId}-${outcomeId}.png"></div>`).join('\n')}
            
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
