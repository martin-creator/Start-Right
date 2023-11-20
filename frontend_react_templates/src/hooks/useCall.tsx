import { useState, useEffect } from 'react';
import axios from 'axios';

interface Farm {
  id: number;
  name: string;
  cow_count: number;
  goat_count: number;
  production: number;
}

interface FarmRequestBody {
  id: number;
  name: string;
  cow_count: number;
  goat_count: number;
  production: number;
}

const useCall = () => {
  const [data, setData] = useState<Farm[] | null>(null);
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(true);

  const fetchData = async () => {
    try {
      const response = await axios.get('http://localhost:8080/api/v1/farms');
      setData(response.data);
    } catch (e) {
      setError(e as any);
    } finally {
      setLoading(false);
    }
  };

  const createFarm = async (body: FarmRequestBody) => {
    try {
      await axios.post('http://localhost:8080/api/v1/farms', body);
    } catch (e) {
      setError(e as any);
    }
  };

  const updateFarm = async (body: FarmRequestBody) => {
    try {
      await axios.put('http://localhost:8080/api/v1/farms', body);
    } catch (e) {
      setError(e as any);
    }
  };

  const getFarmById = async (id: number) => {
    try {
      const response = await axios.get(`http://localhost:8080/api/v1/farms/${id}`);
      setData([response.data]);
    } catch (e) {
      setError(e as any);
    }
  };

  const deleteFarmById = async (id: number) => {
    try {
      await axios.delete(`http://localhost:8080/api/v1/farms/${id}`);
    } catch (e) {
      setError(e as any);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  return { data, error, loading, createFarm, updateFarm, getFarmById, deleteFarmById };
};

export default useCall;