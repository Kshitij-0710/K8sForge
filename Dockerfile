# templates/Dockerfile.node.tpl
# Stage 1: Build stage
FROM node:20-alpine AS build
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .

# Stage 2: Production stage
FROM node:20-alpine
WORKDIR /app
COPY --from=build /app .
EXPOSE 8080
CMD ["node", "index.js"]