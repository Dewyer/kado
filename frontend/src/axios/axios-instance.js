import axios from "axios";

const BASE_URL = process.env.REACT_APP_BASE_URL;

const axiosInstance = axios.create({
	baseURL: BASE_URL,
});

export default axiosInstance;
