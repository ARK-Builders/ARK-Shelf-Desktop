import { open } from '@tauri-apps/api/shell';
import {
  AppBar,
  Button,
  Card,
  CardContent,
  Grid,
  List,
  ListItem,
  ListItemText,
  Toolbar,
  Container,
  Typography,
  TextField,
  Pagination,
  IconButton,
  ButtonGroup,
} from '@mui/material';
import { useState, useEffect, useCallback } from 'react';
import { dialog, invoke, clipboard } from '@tauri-apps/api';
import { ToastContainer, toast, Slide } from 'react-toastify';
import { useForm, SubmitHandler } from 'react-hook-form';
import 'react-toastify/dist/ReactToastify.css';
import dayjs from 'dayjs';
import {
  Add,
  ArrowDownward,
  ArrowUpward,
  CalendarMonth,
  FormatListBulleted,
  LooksOne,
} from '@mui/icons-material';
import useSWR from 'swr';
import { parseDocument } from 'htmlparser2';
import { findOne, getAttributeValue } from 'domutils';
interface LinkInfo {
  title: string;
  desc: string;
  url: string;
  name: string;
  created_time: any;
  score: number;
}
type SortMode = 'normal' | 'date' | 'score';
const Home = () => {
  const [mode, setMode] = useState<SortMode>('normal');
  const [, setLinkNames] = useState<string[]>([]);
  const [linkInfos, setLinkInfos] = useState<LinkInfo[]>([]);
  const { register, handleSubmit, reset } = useForm<LinkInfo>();

  const [page, setPage] = useState(0);
  const itemPerPage = 2;
  const pageCount = Math.ceil(linkInfos.length / itemPerPage);
  const createLink: SubmitHandler<LinkInfo> = (data) => {
    invoke('create_link', {
      ...data,
    }).then(() => {
      // dialog.message('Link Created!');
      refreshInfo();
      toast('Link created!');
      reset();
      // setRefresh(true)
    });
  };

  const refreshInfo = useCallback(async () => {
    const names = (await invoke('read_link_list')) as string[];
    setLinkNames(names);
    const links = await Promise.all(
      names.map(async (val) => {
        const link = {
          ...(await invoke('read_link', { name: val })),
          name: val,
          score: 0,
        } as LinkInfo;
        return link;
      })
    );
    setLinkInfos(links);
  }, []);
  useEffect(() => {
    refreshInfo();
  }, [refreshInfo]);

  interface LinkCardProps {
    link: LinkInfo;
    index: number;
  }
  console.log(linkInfos);
  const LinkCard = ({ link, index }: LinkCardProps) => {
    const { data, error } = useSWR(link.url, (url) => {
      fetch(url).then((data) => {
        data.text().then((val) => {});
      });
    });
    return (
      <ListItem
        dense
        key={link.title}
        secondaryAction={
          <Grid container>
            <Grid item m='auto'>
              <Button
                onClick={() => {
                  clipboard.writeText(link.url);
                }}>
                {'COPY'}
              </Button>
            </Grid>
            <Grid item m='auto' p='auto'>
              <Button
                onClick={() => {
                  open(link.url.toString());
                }}>
                OPEN
              </Button>
              <Button
                onClick={() => {
                  invoke('delete_link', {
                    name: link.name,
                  });
                  refreshInfo();
                  toast('Link deleted!');
                }}
                color='error'>
                DELETE
              </Button>
              <IconButton
                color='primary'
                onClick={() => {
                  let linksInfo = linkInfos.map((val, idx) => {
                    if (index === idx) {
                      val.score += 1;
                    }
                    return val;
                  });
                  setLinkInfos(linksInfo);
                }}>
                <ArrowUpward />
              </IconButton>
              <IconButton
                color='error'
                onClick={() => {
                  let linksInfo = linkInfos.map((val, idx) => {
                    if (index === idx) {
                      val.score -= 1;
                    }
                    return val;
                  });
                  setLinkInfos(linksInfo);
                }}>
                <ArrowDownward />
              </IconButton>
            </Grid>
          </Grid>
        }>
        <ListItemText
          primary={<Typography variant='h6'>{link.title}</Typography>}
          secondary={
            <>
              <Typography variant='subtitle2'>{link.desc}</Typography>
              <Typography variant='body2'>
                {dayjs
                  .unix(link.created_time.secs_since_epoch)
                  .toDate()
                  .toLocaleString()}
              </Typography>
            </>
          }></ListItemText>
      </ListItem>
    );
  };

  return (
    <>
      <AppBar position='fixed'>
        <Toolbar>
          <Typography
            variant='h6'
            sx={{
              flexGrow: 1,
            }}>
            ARK Shelf
          </Typography>
        </Toolbar>
      </AppBar>
      <Toolbar />
      <Container
        sx={{
          mt: 2,
        }}>
        <Grid container spacing={8}>
          <Grid item xs={8}>
            <Card>
              <CardContent>
                <List>
                  {linkInfos
                    .sort((a, b) => {
                      switch (mode) {
                        case 'normal':
                          return a.title.localeCompare(b.title);
                        case 'date':
                          return (
                            b.created_time.secs_since_epoch -
                            a.created_time.secs_since_epoch
                          );
                        case 'score':
                          return b.score - a.score;
                      }
                    })
                    .slice(itemPerPage * page, itemPerPage * (page + 1))
                    .map((val, idx) => {
                      return <LinkCard link={val} index={idx} />;
                    })}
                </List>
                <Pagination
                  count={pageCount === 0 ? 1 : pageCount}
                  page={page + 1}
                  onChange={(_, page) => {
                    // mui pages are started from 1, against to zero-based index array
                    setPage(page - 1);
                  }}
                  showFirstButton
                  showLastButton
                />
              </CardContent>
            </Card>
          </Grid>
          <Grid item xs={4}>
            <Grid item>
              <ButtonGroup>
                <IconButton
                  onClick={() => {
                    setMode('normal');
                  }}>
                  <FormatListBulleted />
                </IconButton>

                <IconButton onClick={() => setMode('date')}>
                  <CalendarMonth />
                </IconButton>

                <IconButton onClick={() => setMode('score')}>
                  <LooksOne />
                </IconButton>
              </ButtonGroup>
            </Grid>
            <form onSubmit={handleSubmit(createLink)}>
              <TextField
                fullWidth
                label='url'
                margin='normal'
                {...register('url', { required: true })}></TextField>
              <TextField
                fullWidth
                label='title'
                margin='normal'
                {...register('title', { required: true })}></TextField>
              <TextField
                fullWidth
                label='description(optional)'
                margin='normal'
                {...register('desc', {
                  required: false,
                  value: '',
                })}></TextField>

              <Button type='submit'>Create</Button>
            </form>
          </Grid>
        </Grid>
      </Container>
      <ToastContainer
        position='bottom-right'
        autoClose={1000}
        hideProgressBar
        newestOnTop={false}
        closeOnClick
        rtl={false}
        pauseOnFocusLoss
        draggable
        pauseOnHover
        transition={Slide}
        theme='dark'
      />
    </>
  );
};
export default Home;
