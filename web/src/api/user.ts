import { AxiosResponse } from "axios";
import { http, api } from "@/utils/http";
import { ResponsePromise } from "ky";

export const userLogin = (): ResponsePromise => api.get("/user/logion");
