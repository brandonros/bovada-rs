import express from 'express'
import fs from 'fs'
import { execa } from 'execa'
import assert from 'assert'

const extractOutcomes = async (eventId) => {
    const eventSlug = process.env.EVENT_SLUG
    const response = await fetch(`https://www.bovada.lv/services/sports/event/coupon/events/A/description/${eventSlug}?lang=en`)
    assert(response.status === 200)
    const responseBody = await response.json()
    const events = responseBody[0].events
    const event = events.find(event => event.id === eventId)
    const displayGroups = event.displayGroups
    const gameLinesDisplayGroup = displayGroups.find(displayGroup => displayGroup.description === 'Game Lines')
    const outcomesMap = {}
    const moneylineMarket = gameLinesDisplayGroup.markets.find(market => market.description === 'Moneyline')
    if (moneylineMarket && moneylineMarket.period.description === 'Live Game') {
        for (const outcome of moneylineMarket.outcomes) {
            outcomesMap[outcome.id] = {
                market: moneylineMarket,
                outcome
            }
        }
    }
    const pointSpreadMarket = gameLinesDisplayGroup.markets.find(market => market.description === 'Point Spread')
    if (pointSpreadMarket && pointSpreadMarket.period.description === 'Live Game') {
        for (const outcome of pointSpreadMarket.outcomes) {
            outcomesMap[outcome.id] = {
                market: pointSpreadMarket,
                outcome
            }
        }
    }
    const totalSpreadMarket = gameLinesDisplayGroup.markets.find(market => market.description === 'Total')
    if (totalSpreadMarket && totalSpreadMarket.period.description === 'Live Game') {
        for (const outcome of totalSpreadMarket.outcomes) {
            outcomesMap[outcome.id] = {
                market: totalSpreadMarket,
                outcome
            }
        }
    }
    return outcomesMap
}

const main = async () => {
    const app = express()
    app.use('/generated', express.static('../generated', { etag: false }))
    app.get('/events/:eventId/:type', async (req, res) => {
        try {
            const { eventId, type } = req.params
            const outcomesMap = await extractOutcomes(eventId)
            const outcomeIds = Object.keys(outcomesMap)
            assert(outcomeIds.length > 0)
            for (const outcomeId of outcomeIds) {
                await execa('./scripts/extract.py', [eventId, outcomeId], { cwd: '../' })
                await execa(`./scripts/plot-${type}.sh`, [eventId, outcomeId], { cwd: '../' })
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
                            }, 10000)
                        </script>
                    </body>
                </html>
            `
            res
                .status(200)
                .set('Content-Type', 'text/html')
                .send(html)
        } catch (err) {
            console.error(err)
            res
                .status(500)
                .set('Content-Type', 'text/html')
                .send(err)
        }
    })
    await new Promise(resolve => app.listen(8080, resolve))
}

main()
