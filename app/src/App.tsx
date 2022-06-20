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
  Tooltip,
} from '@mui/material';
import { useState, useEffect, useCallback, useReducer } from 'react';
import { invoke, clipboard } from '@tauri-apps/api';
import { ToastContainer, toast, Slide } from 'react-toastify';
import { useForm, SubmitHandler } from 'react-hook-form';
import 'react-toastify/dist/ReactToastify.css';
import dayjs from 'dayjs';
import {
  ArrowDownward,
  ArrowUpward,
  CalendarMonth,
  FormatListBulleted,
  LooksOne,
} from '@mui/icons-material';

interface LinkInfo {
  title: string;
  desc: string;
  url: string;
  name: string;
  created_time: any;
  score: number;
}
interface OpenGraph {
  /// Represents the "og:title" OpenGraph meta tag.
  ///
  /// The title of your object as it should appear within
  /// the graph, e.g., "The Rock".
  title: string;
  /// Represents the "og:description" OpenGraph meta tag
  description: string;
  /// Represents the "og:url" OpenGraph meta tag
  url: string;
  /// Represents the "og:image" OpenGraph meta tag
  image: string;
  /// Represents the "og:type" OpenGraph meta tag
  ///
  /// The type of your object, e.g., "video.movie". Depending on the type
  /// you specify, other properties may also be required.
  object_type: string;
  /// Represents the "og:locale" OpenGraph meta tag
  locale: string;
}
interface LinkListItemProps {
  link: LinkInfo;
  index: number;
}
type SortMode = 'normal' | 'date' | 'score';
const Home = () => {
  const [mode, setMode] = useState<SortMode>('normal');
  const [, setLinkNames] = useState<string[]>([]);
  const [linkInfos, setLinkInfos] = useState<LinkInfo[]>([]);
  const { register, handleSubmit, reset } = useForm<LinkInfo>();

  const [page, setPage] = useState(0);
  console.log(linkInfos);
  const itemPerPage = 5;
  const pageCount = Math.ceil(linkInfos.length / itemPerPage);
  const createLink: SubmitHandler<LinkInfo> = (data) => {
    invoke('create_link', {
      ...data,
    }).then(() => {
      refreshInfo();
      toast('Link created!');
      reset();
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
    setLinkInfos(
      (links as LinkInfo[])
        // eslint-disable-next-line array-callback-return
        .sort((a, b) => {
          switch (mode) {
            case 'normal':
              return a.title.localeCompare(b.title);
            case 'date':
              return (
                b.created_time.secs_since_epoch -
                a.created_time.secs_since_epoch
              );
            // Default to score
            case 'score':
              return 0;
          }
        })
    );
  }, [mode]);

  useEffect(() => {
    refreshInfo();
  }, [refreshInfo]);
  interface LinkListProps {
    linkInfos: LinkInfo[];
  }
  const LinkList = ({ linkInfos }: LinkListProps) => {
    const [scoreLinkInfos, setScoreLinkInfos] = useState<LinkInfo[]>(linkInfos);

    const LinkListItem = ({ link, index }: LinkListItemProps) => {
      const [previewInfo, setPreviewInfo] = useState<OpenGraph>();

      useEffect(() => {
        invoke('generate_link_preview', {
          url: link.url.toString(),
        }).then((val) => setPreviewInfo(val as OpenGraph));
      }, [link.url]);

      return (
        <Tooltip
          arrow
          title={
            <>
              <Typography variant='body2'>
                {previewInfo?.title ??
                  'Preview may not available at the moment'}
              </Typography>
              <img
                loading='lazy'
                alt='preview'
                src={previewInfo?.image}
                width={250}></img>
            </>
          }>
          <ListItem
            dense
            key={index}
            sx={{
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'flex-start',
            }}>
            <ListItemText
              primary={
                <Typography
                  sx={{
                    maxWidth: '25rem',
                  }}
                  paragraph>
                  {link.title}
                </Typography>
              }
              secondary={dayjs
                .unix(link.created_time.secs_since_epoch)
                .toDate()
                .toLocaleString()}></ListItemText>

            <Grid container>
              <Grid item container m='auto'>
                <Button
                  onClick={() => {
                    clipboard.writeText(link.url);
                  }}>
                  {'COPY'}
                </Button>
                {/* </Grid>
            <Grid item m='auto' p='auto'> */}
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
                  disabled={index === 0}
                  onClick={() => {
                    let arr = Array.from(scoreLinkInfos);
                    [arr[index - 1], arr[index]] = [arr[index], arr[index - 1]];
                    console.log(arr);
                    setScoreLinkInfos(arr);
                  }}>
                  <ArrowUpward />
                </IconButton>
                <IconButton
                  color='error'
                  disabled={index === linkInfos.length - 1}
                  onClick={() => {
                    let arr = Array.from(scoreLinkInfos);
                    [arr[index], arr[index + 1]] = [arr[index + 1], arr[index]];
                    console.log(arr);
                    setScoreLinkInfos(arr);
                  }}>
                  <ArrowDownward />
                </IconButton>
              </Grid>
            </Grid>
          </ListItem>
        </Tooltip>
      );
    };
    return (
      <List>
        {scoreLinkInfos
          .map((val, idx, arr) => {
            // console.log(val, arr);
            return (
              <div id={val.title} key={val.title}>
                <LinkListItem link={val} index={idx} />
              </div>
            );
          })
          .slice(itemPerPage * page, itemPerPage * (page + 1))}
      </List>
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
                <LinkList linkInfos={linkInfos} />

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
                <Tooltip title={'Normal mode'}>
                  <IconButton
                    onClick={() => {
                      setMode('normal');
                    }}>
                    <FormatListBulleted />
                  </IconButton>
                </Tooltip>

                <Tooltip title={'Sorting By Date'}>
                  <IconButton onClick={() => setMode('date')}>
                    <CalendarMonth />
                  </IconButton>
                </Tooltip>
                <Tooltip title={'Sorting By Score'}>
                  <IconButton onClick={() => setMode('score')}>
                    <LooksOne />
                  </IconButton>
                </Tooltip>
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
