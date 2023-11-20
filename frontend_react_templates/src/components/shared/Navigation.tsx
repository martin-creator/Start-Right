import React, { useState } from 'react';
import Logo from "./Logo";

type Props = {
  currentPage: string;
  setCurrentPage: (page: string) => void;
}

const Navigation: React.FC<Props> = ({ currentPage, setCurrentPage }) => {
  const [isOpen, setIsOpen] = useState(false);

  const pages = ["home_page", "farm_detail_page"];

  const handlePageChange = (page: string) => {
    setCurrentPage(page);
    setIsOpen(false);
  }

  return (
    <nav className="bg-white px-6 py-4 shadow">
      <div className="flex flex-col md:flex-row md:justify-between md:items-center">
        <div className="flex justify-between items-center">
          <div>
            <Logo />
          </div>
          <div className="md:hidden">
            <button type="button" className="text-gray-500 hover:text-gray-600 focus:outline-none focus:text-gray-600" aria-label="toggle menu" onClick={() => setIsOpen(!isOpen)}>
              <svg viewBox="0 0 24 24" className="h-6 w-6 fill-current">
                <path fillRule="evenodd" d="M4 5h16a1 1 0 010 2H4a1 1 0 110-2zm0 6h16a1 1 0 010 2H4a1 1 0 110-2zm0 6h16a1 1 0 010 2H4a1 1 0 110-2z"></path>
              </svg>
            </button>
          </div>
        </div>
        <div className={`md:flex ${isOpen ? 'block' : 'hidden'}`}>
          {pages.map((page, index) => (
            <a key={index} onClick={() => handlePageChange(page)} className={`mt-3 md:mt-0 md:ml-6 text-gray-600 hover:text-gray-700 ${currentPage === page ? 'text-blue-700' : ''}`}>{page}</a>
          ))}
        </div>
      </div>
    </nav>
  );
}

export default Navigation;