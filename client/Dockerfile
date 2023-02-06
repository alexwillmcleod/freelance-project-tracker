FROM node:latest

WORKDIR /usr/src/app

COPY package*.json ./

# Install deps for prod
RUN yarn install

# Bundle app source
COPY . .

EXPOSE 5173

CMD ["yarn", "run", "dev", "--host"]