import fs from 'fs'
import express from 'express'

const main = async () => {
    const app = express()
    app.use('/generated', express.static('../generated'))
    app.get('/', (req, res) => {
        const template = fs.readFileSync('./static/index.html', 'utf8')
        const outcomeId1 = req.query.outcomeId1
        const outcomeId2 = req.query.outcomeId2
        res
            .status(200)
            .set('Content-Type', 'text/html')
            .send(template
                .replaceAll('{{OUTCOME_ID_1}}', outcomeId1)
                .replaceAll('{{OUTCOME_ID_2}}', outcomeId2))
    })
    await new Promise(resolve => app.listen(8080, resolve))
}

main()
