# If you update this, make sure you update the .nvmrc too
FROM node:16.13.1-alpine AS builder

COPY ./api/schema.graphql /app/ui/
COPY ./ui/ /app/ui/
WORKDIR /app/ui
RUN npm install
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/ui/build /app/laulud
COPY ./ui/nginx.conf /etc/nginx/nginx.conf
