import { useState, useEffect } from 'react';
import axios from 'axios';

interface NutritionPostRequestBody {
  id: number;
  name: string;
  requirements: string;
  allergies: string;
}

interface NutritionGetResponseBody {
  id: number;
  name: string;
  requirements: string;
  allergies: string;
}

interface NutritionPutRequestBody {
  id: number;
  name: string;
  requirements: string;
  allergies: string;
}

interface NutritionIdGetResponseBody {
  id: number;
  name: string;
  requirements: string;
  allergies: string;
}

interface AuthRegisterPostRequestBody {
  id: number;
  username: string;
  password: string;
}

interface AuthLoginPostRequestBody {
  username: string;
  password: string;
}

const useCall = () => {
  const [data, setData] = useState<any>(null);
  const [error, setError] = useState<any>(null);

  const callApi = async (
    route: string,
    method: string,
    requestBody: any
  ) => {
    try {
      const response = await axios({
        method,
        url: `http://localhost:8080${route}`,
        data: requestBody,
      });
      setData(response.data);
    } catch (e) {
      setError(e as any);
    }
  };

  useEffect(() => {
    // Call APIs here
    callApi('/api/v1/nutrition', 'post', {
      id: 1,
      name: 'Test',
      requirements: 'Test',
      allergies: 'Test',
    } as NutritionPostRequestBody);

    callApi('/api/v1/nutrition', 'get', null);

    callApi('/api/v1/nutrition', 'put', {
      id: 1,
      name: 'Test',
      requirements: 'Test',
      allergies: 'Test',
    } as NutritionPutRequestBody);

    callApi('/api/v1/nutrition/1', 'get', null);

    callApi('/api/v1/nutrition/1', 'delete', null);

    callApi('/api/v1/auth/register', 'post', {
      id: 1,
      username: 'Test',
      password: 'Test',
    } as AuthRegisterPostRequestBody);

    callApi('/api/v1/auth/login', 'post', {
      username: 'Test',
      password: 'Test',
    } as AuthLoginPostRequestBody);
  }, []);

  return { data, error };
};

export default useCall;