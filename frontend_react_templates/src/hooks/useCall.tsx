import { useState, useEffect } from 'react';
import axios from 'axios';

interface StationRequestBody {
  id: number;
  name: string;
  trains: object;
}

interface StationResponse {
  id: number;
  name: string;
  trains: object;
}

const useCall = () => {
  const [data, setData] = useState<StationResponse | null>(null);
  const [error, setError] = useState<any>(null);

  const postStation = async (body: StationRequestBody) => {
    try {
      await axios.post('http://localhost:8080/api/v1/stations', body);
    } catch (e) {
      setError(e as any);
    }
  };

  const getStations = async () => {
    try {
      const response = await axios.get('http://localhost:8080/api/v1/stations');
      setData(response.data);
    } catch (e) {
      setError(e as any);
    }
  };

  const putStation = async (body: StationRequestBody) => {
    try {
      await axios.put('http://localhost:8080/api/v1/stations', body);
    } catch (e) {
      setError(e as any);
    }
  };

  const getStation = async (id: number) => {
    try {
      const response = await axios.get(`http://localhost:8080/api/v1/stations/${id}`);
      setData(response.data);
    } catch (e) {
      setError(e as any);
    }
  };

  const deleteStation = async (id: number) => {
    try {
      await axios.delete(`http://localhost:8080/api/v1/stations/${id}`);
    } catch (e) {
      setError(e as any);
    }
  };

  useEffect(() => {
    getStations();
  }, []);

  return { data, error, postStation, getStations, putStation, getStation, deleteStation };
};

export default useCall;