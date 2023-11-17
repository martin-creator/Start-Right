import { useState, useEffect } from 'react';
import axios from 'axios';

interface Meeting {
  id: number;
  name: string;
  completed: boolean;
}

interface User {
  id: number;
  username: string;
  password: string;
}

const useCall = () => {
  const [data, setData] = useState<Meeting[] | Meeting | null>(null);
  const [error, setError] = useState<any>(null);

  const postMeeting = async (meeting: Meeting) => {
    try {
      await axios.post('http://localhost:8080/api/v1/meetings', meeting);
    } catch (e) {
      setError(e as any);
    }
  };

  const getMeetings = async () => {
    try {
      const response = await axios.get('http://localhost:8080/api/v1/meetings');
      setData(response.data);
    } catch (e) {
      setError(e as any);
    }
  };

  const putMeeting = async (meeting: Meeting) => {
    try {
      await axios.put('http://localhost:8080/api/v1/meetings', meeting);
    } catch (e) {
      setError(e as any);
    }
  };

  const getMeetingById = async (id: number) => {
    try {
      const response = await axios.get(`http://localhost:8080/api/v1/meetings/${id}`);
      setData(response.data);
    } catch (e) {
      setError(e as any);
    }
  };

  const deleteMeetingById = async (id: number) => {
    try {
      await axios.delete(`http://localhost:8080/api/v1/meetings/${id}`);
    } catch (e) {
      setError(e as any);
    }
  };

  const register = async (user: User) => {
    try {
      await axios.post('http://localhost:8080/api/v1/auth/register', user);
    } catch (e) {
      setError(e as any);
    }
  };

  const login = async (user: User) => {
    try {
      await axios.post('http://localhost:8080/api/v1/auth/login', user);
    } catch (e) {
      setError(e as any);
    }
  };

  useEffect(() => {
    getMeetings();
  }, []);

  return { data, error, postMeeting, getMeetings, putMeeting, getMeetingById, deleteMeetingById, register, login };
};

export default useCall;