import { useState, useEffect } from 'react';
import axios from 'axios';

interface QuestionRequestBody {
  id: number;
  question: string;
  options: string[];
  answer: string;
}

interface QuestionResponse {
  id: number;
  question: string;
  options: string[];
  answer: string;
}

interface RegisterRequestBody {
  id: number;
  username: string;
  password: string;
  score: number;
}

interface LoginRequestBody {
  username: string;
  password: string;
}

const useCall = () => {
  const [data, setData] = useState<any>(null);
  const [error, setError] = useState<any>(null);

  const postQuestion = async (body: QuestionRequestBody) => {
    try {
      const response = await axios.post('http://localhost:8080/api/v1/questions', body);
      setData(response.data);
    } catch (e) {
      setError(e as any);
    }
  };

  const getQuestions = async () => {
    try {
      const response = await axios.get('http://localhost:8080/api/v1/questions');
      setData(response.data as QuestionResponse[]);
    } catch (e) {
      setError(e as any);
    }
  };

  const putQuestion = async (body: QuestionRequestBody) => {
    try {
      const response = await axios.put('http://localhost:8080/api/v1/questions', body);
      setData(response.data);
    } catch (e) {
      setError(e as any);
    }
  };

  const getQuestionById = async (id: number) => {
    try {
      const response = await axios.get(`http://localhost:8080/api/v1/questions/${id}`);
      setData(response.data as QuestionResponse);
    } catch (e) {
      setError(e as any);
    }
  };

  const deleteQuestionById = async (id: number) => {
    try {
      const response = await axios.delete(`http://localhost:8080/api/v1/questions/${id}`);
      setData(response.data);
    } catch (e) {
      setError(e as any);
    }
  };

  const register = async (body: RegisterRequestBody) => {
    try {
      const response = await axios.post('http://localhost:8080/auth/register', body);
      setData(response.data);
    } catch (e) {
      setError(e as any);
    }
  };

  const login = async (body: LoginRequestBody) => {
    try {
      const response = await axios.post('http://localhost:8080/auth/login', body);
      setData(response.data);
    } catch (e) {
      setError(e as any);
    }
  };

  const updateScore = async (username: string, score: number) => {
    try {
      const response = await axios.put(`http://localhost:8080/users/${username}/score/${score}`);
      setData(response.data);
    } catch (e) {
      setError(e as any);
    }
  };

  return {
    data,
    error,
    postQuestion,
    getQuestions,
    putQuestion,
    getQuestionById,
    deleteQuestionById,
    register,
    login,
    updateScore,
  };
};

export default useCall;