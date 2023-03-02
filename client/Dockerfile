# FROM node:latest as build

# RUN mkdir usr/src/app
# WORKDIR /usr/src/app

# ENV PATH /usr/src/app/node_modules/.bin:$PATH
# COPY package.json /usr/src/app/package.json

# RUN rm -rf node_modules

# # Install deps for prod
# RUN npm install

# COPY . . 

# FROM nginx:alpine

# # Remove default nginx config
# RUN rm /etc/nginx/conf.d/default.conf

# # Add nginx config
# COPY nginx.conf /etc/nginx/nginx.conf

# COPY --from=build /usr/src/app/dist /usr/share/nginx/html

# EXPOSE 80

# CMD ["nginx", "-g", "daemon off;"]

FROM node:latest

WORKDIR /usr/src/app

COPY package*.json ./

RUN yarn install

COPY . . 

EXPOSE 5173 

CMD ["yarn", "run", "dev", "--host"]