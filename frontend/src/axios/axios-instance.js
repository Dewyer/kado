import axios from "axios";

const BASE_URL = process.env.REACT_APP_BASE_URL;
const API_KEY = process.env.REACT_APP_API_KEY;

const axiosInstance = axios.create({
	baseURL: BASE_URL,
	headers: { "X-Api-Key": API_KEY },
});

export default axiosInstance;
