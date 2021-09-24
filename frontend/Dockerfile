FROM node:alpine
EXPOSE 3000
WORKDIR /app
COPY package.json ./
COPY yarn.lock ./
COPY ./ ./
RUN yarn install
CMD ["yarn", "start"]