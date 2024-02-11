import express from 'express'
import { execa } from 'execa'
import assert from 'assert'
import dotenv from 'dotenv'

const buildOutcomesMap = async (eventId, periodDescription) => {
    const eventSlug = process.env.EVENT_SLUG
    const requestUrl = `https://www.bovada.lv/services/sports/event/coupon/events/A/description/${eventSlug}?lang=en`
    console.log({
        requestUrl
    })
    const response = await fetch(requestUrl)
    assert(response.status === 200)
    const responseBody = await response.json()
    const coupons = responseBody
    assert(coupons.length === 1)
    const coupon = coupons[0]
    const events = coupon.events
    const event = events.find(event => event.id === eventId)
    const displayGroups = event.displayGroups
    const gameLinesDisplayGroup = displayGroups.find(displayGroup => displayGroup.description === 'Game Lines')
    const outcomesMap = {}
    const moneylineMarket = gameLinesDisplayGroup.markets.find(market => market.description === 'Moneyline')
    if (moneylineMarket && moneylineMarket.period.description === periodDescription) {
        for (const outcome of moneylineMarket.outcomes) {
            outcomesMap[outcome.id] = {
                market: moneylineMarket,
                outcome
            }
        }
    }
    const pointSpreadMarket = gameLinesDisplayGroup.markets.find(market => market.description === 'Point Spread')
    if (pointSpreadMarket && pointSpreadMarket.period.description === periodDescription) {
        for (const outcome of pointSpreadMarket.outcomes) {
            outcomesMap[outcome.id] = {
                market: pointSpreadMarket,
                outcome
            }
        }
    }
    const totalSpreadMarket = gameLinesDisplayGroup.markets.find(market => market.description === 'Total')
    if (totalSpreadMarket && totalSpreadMarket.period.description === periodDescription) {
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
    dotenv.config({ path: '../.env' })
    const app = express()
    app.use('/generated', express.static('../generated', { etag: false }))
    app.get('/events/:eventId/:type', async (req, res) => {
        try {
            const { eventId, type } = req.params
            const periodDescription = req.query.periodDescription || 'Live Game'
            const outcomesMap = await buildOutcomesMap(eventId, periodDescription)
            const outcomeIds = Object.keys(outcomesMap)
            assert(outcomeIds.length > 0)
            const order = {
                'Moneyline': 1,
                'Point Spread': 2,
                'Total': 3
            }
            outcomeIds.sort((a, b) => {
                const aOutcome = outcomesMap[a]
                const bOutcome = outcomesMap[b]
                const aMarketDescription = aOutcome.market.description
                const bMarketDescription = bOutcome.market.description
                const aOrder = order[aMarketDescription]
                const bOrder = order[bMarketDescription]
                return aOrder - bOrder
            })
            for (const outcomeId of outcomeIds) {
                const outcome = outcomesMap[outcomeId]
                const outcomeMarketDescription = outcome.market.description
                const showY2 = outcomeMarketDescription === 'Moneyline' ? 'false' : 'true'
                await execa('./scripts/extract.js', [eventId, outcomeId], { cwd: '../' })
                await execa(`./scripts/plot-${type}.sh`, [
                    eventId,
                    outcomeId,
                    showY2
                ], { cwd: '../' })
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
                .set('Content-Type', 'text/plain')
                .send(err)
        }
    })
    await new Promise(resolve => app.listen(8080, resolve))
    console.log('listening on 8080')
}

main()
