import React, { useEffect, useState } from 'react';
import axios from 'axios';
import 'tailwindcss/tailwind.css';

function PageName() {
  const [questions, setQuestions] = useState([]);
  const [question, setQuestion] = useState(null);
  const [error, setError] = useState(null);

  const fetchQuestions = async () => {
    try {
      const response:any = await axios.get('/api/v1/questions');
      setQuestions(response.data);
    } catch (e:any) {
      setError(e.message);
    }
  };

  const fetchQuestion = async (id:any) => {
    try {
      const response:any = await axios.get(`/api/v1/questions/${id}`);
      setQuestion(response.data);
    } catch (e:any) {
      setError(e.message);
    }
  };

  const createQuestion = async (questionData:any) => {
    try {
      await axios.post('/api/v1/questions', questionData);
      fetchQuestions();
    } catch (e:any) {
      setError(e.message);
    }
  };

  const updateQuestion = async (id:any, questionData:any) => {
    try {
      await axios.put(`/api/v1/questions/${id}`, questionData);
      fetchQuestions();
    } catch (e:any) {
      setError(e.message);
    }
  };

  const deleteQuestion = async (id:any) => {
    try {
      await axios.delete(`/api/v1/questions/${id}`);
      fetchQuestions();
    } catch (e) {
      setError(e.message);
    }
  };

  useEffect(() => {
    fetchQuestions();
  }, []);

  return (
    <div className="font-sans text-gray-900 antialiased">
      <section className="flex flex-col items-center justify-center min-h-screen py-12 px-4 sm:px-6 lg:px-8">
        <div className="progress_section flex flex-col items-center justify-center text-center">
            <h2 className="text-2xl font-extrabold text-gray-900">Your Progress</h2>
            <p className="mt-2 text-sm text-gray-600">You are currently on question 5 of 10.</p>
        </div>
        <div className="question_section flex flex-col items-center justify-center text-center mt-8">
            <h2 className="text-2xl font-extrabold text-gray-900">Current Question</h2>
            <p className="mt-2 text-sm text-gray-600">What is the capital of France?</p>
            <div className="answers flex flex-col items-center justify-center mt-4">
                <button className="answer bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">A. Paris</button>
                <button className="answer bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mt-2">B. London</button>
                <button className="answer bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mt-2">C. Rome</button>
                <button className="answer bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mt-2">D. Madrid</button>
            </div>
        </div>
        <div className="score_section flex flex-col items-center justify-center text-center mt-8">
            <h2 className="text-2xl font-extrabold text-gray-900">Your Score</h2>
            <p className="mt-2 text-sm text-gray-600">You have correctly answered 3 out of 4 questions so far.</p>
        </div>
      </section>
    </div>
  );
}

export default PageName;