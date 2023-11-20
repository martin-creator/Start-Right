type Props = {
  currentPage: string;
  setCurrentPage: (page: string) => void;
}

const Footer: React.FC<Props> = ({setCurrentPage, currentPage}) => {
  const pages = ["home_page", "user_dashboard"];
  const colors = ["#32a852", "#0fa0d1", "#d10fcb"];

  const handleClick = (page: string) => {
    setCurrentPage(page);
  }

  return (
    <div className="fixed bottom-0 w-full bg-gray-200 p-4 flex justify-around items-center">
      {pages.map((page, index) => (
        <div key={index} onClick={() => handleClick(page)} style={{color: currentPage === page ? colors[index % colors.length] : 'black'}}>
          {page}
        </div>
      ))}
    </div>
  )
}

export default Footer;