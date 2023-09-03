import { Link } from "react-router-dom";
import { styled } from "styled-components";

const TabButton = styled.button<{ isCurrentTab?: boolean }>(
  ({ isCurrentTab }) => ({
    background: isCurrentTab ? "#ddd" : "white",
    margin: 0,
    borderRadius: 0,
    outline: "none",
  })
);

const TabsWrapper = styled.div(() => ({
  marginBottom: "40px",
}));

interface ITabData {
    path: string,
    name: string,
    isCurrentTab?: boolean,
}

const Tabs = ({data}: {data: ITabData[]}) => {
  return (
    <TabsWrapper>
      {
        data.map(tabData => (
            <Link to={tabData.path}>
                <TabButton isCurrentTab={!!tabData.isCurrentTab}>{tabData.name}</TabButton>
            </Link>
        ))
      }
    </TabsWrapper>
  );
};

export default Tabs;
