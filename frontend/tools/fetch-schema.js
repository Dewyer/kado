const axios = require("axios");
const https = require("https");
const fs = require("fs");
const path = require("path");

const instance = axios.create({
	httpsAgent: new https.Agent({
		rejectUnauthorized: false,
	}),
});

function addOperationIdsToSchema(schema) {
	const data = schema;

	Object.keys(data.paths).forEach((endpointPath) => {
		const operations = Object.keys(data.paths[endpointPath]);

		operations.forEach((operation) => {
			const oprationName = endpointPath.replace("/api/", "").replace(/\//g, "");
			data.paths[endpointPath][operation].operationId = oprationName;
		});
	});

	return data;
}

const url = `http://localhost:3001/api/openapi.json`;
const filePath = `../src/typings/schemas/api-schema.json`;
const message = `==> Api schema fetched successfully...`;
instance
	.get(url)
	// eslint-disable-next-line no-loop-func
	.then((response) => {
		const updatedSchema = addOperationIdsToSchema(response.data);
		fs.writeFileSync(path.resolve(__dirname, filePath), JSON.stringify(updatedSchema, null, 2));

		// eslint-disable-next-line no-console
		console.log(message);
	})
	// eslint-disable-next-line no-console
	.catch(console.error);
