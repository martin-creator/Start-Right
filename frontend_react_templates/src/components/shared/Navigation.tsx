import React, { useState } from 'react';
import Logo from "./Logo";

type Props = {
  currentPage: string;
  setCurrentPage: (page: string) => void;
}

const Navigation: React.FC<Props> = ({setCurrentPage, currentPage}) => {
  const [isOpen, setIsOpen] = useState(false);

  const handleMenuClick = () => {
    setIsOpen(!isOpen);
  }

  const handleLinkClick = (page: string) => {
    setCurrentPage(page);
    setIsOpen(false);
  }

  return (
    <nav className="bg-white px-6 py-4 shadow">
      <div className="flex flex-col md:flex-row justify-between items-center">
        <div className="flex justify-between items-center">
          <div>
            <Logo />
          </div>
          <div className="md:hidden">
            <button type="button" className="text-gray-500 hover:text-gray-600 focus:outline-none focus:text-gray-600" aria-label="toggle menu" onClick={handleMenuClick}>
              <svg viewBox="0 0 24 24" className="h-6 w-6 fill-current">
                <path fillRule="evenodd" d={isOpen ? "M6 18L18 6M6 6l12 12" : "M4 6h16M4 12h16M4 18h16"} />
              </svg>
            </button>
          </div>
        </div>
        <div className={`md:flex items-center ${isOpen ? 'block' : 'hidden'}`}>
          <div className="flex flex-col md:flex-row md:mx-6">
            <a className={`my-1 text-sm text-gray-700 font-medium hover:text-indigo-500 md:mx-4 md:my-0 ${currentPage === 'home_page' ? 'text-indigo-500' : ''}`} 
               href="#" onClick={() => handleLinkClick('home_page')}>Home</a>
            <a className={`my-1 text-sm text-gray-700 font-medium hover:text-indigo-500 md:mx-4 md:my-0 ${currentPage === 'user_dashboard' ? 'text-indigo-500' : ''}`} 
               href="#" onClick={() => handleLinkClick('user_dashboard')}>Dashboard</a>
          </div>
        </div>
      </div>
    </nav>
  );
}

export default Navigation;