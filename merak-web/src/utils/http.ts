import axios from "axios";
import ky from "ky";
import { InternalAxiosRequestConfig, AxiosResponse } from "axios";

const http = axios.create({
  baseURL: "",
  timeout: 5000,
  headers: {
    "Content-Type": "application/json",
  },
});

http.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    console.log(config.baseURL);
    return config;
  },
  (error: any) => {
    console.error(error);
    return Promise.reject(error);
  }
);

http.interceptors.response.use(
  (resp: AxiosResponse) => {
    return resp;
  },
  (error: any) => {
    console.error("resp error: ", error);
    return Promise.reject(error);
  }
);

const api = ky.create({
  prefixUrl: "",
  headers: {
    "Content-Type": "application/json",
  },
  hooks: {
    beforeRequest: [
      (request) => {
        request.headers.set("X-Requested-With", "ky");
        console.log(request.headers);
      },
    ],
    afterResponse: [
      (_request, _options, response) => {
        // You could do something with the response, for example, logging.
        console.log(response);
      },
    ],
  },
});

export { api, http };
